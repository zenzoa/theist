use std::{ str, error::Error };
use bytes::{ Bytes, BytesMut };

use super::file_block::File;
use super::pray::{
	Block,
	BlockHeader,
	read_tag_block,
	write_block_header,
	write_tag_block,
	get_dependencies,
	write_dependencies,
	file_stem
};

#[derive(Clone, serde::Serialize)]
pub struct Egg {
	pub name: String,
	pub genetics_file: String,
	pub genetics_file_mother: String,
	pub genetics_file_father: String,
	pub sprite_file_male: String,
	pub sprite_file_female: String,
	pub animation_string: String,
	pub dependencies: Vec<String>,
}

pub fn read_egg_block(buffer: &mut Bytes, name: &str) -> Result<Vec<Block>, Box<dyn Error>> {
	let tag = read_tag_block(buffer)?;
	Ok(vec![Block::Egg(Egg {
		name: name.to_string(),
		genetics_file: genetics_file_name(tag.str_values.get("Genetics File")),
		genetics_file_mother: genetics_file_name(tag.str_values.get("Mother Genetic File")),
		genetics_file_father: genetics_file_name(tag.str_values.get("Father Genetic File")),
		sprite_file_male: tag.str_values.get("Egg Glyph File").unwrap_or(&String::new()).clone(),
		sprite_file_female: tag.str_values.get("Egg Glyph File 2").unwrap_or(&String::new()).clone(),
		animation_string: tag.str_values.get("Egg Animation String").unwrap_or(&String::new()).clone(),
		dependencies: get_dependencies(&tag),
	})])
}

fn genetics_file_name(file_stem: Option<&String>) -> String {
	match file_stem {
		Some(file_name) => format!("{}.gen", file_name.trim_end_matches('*')),
		None => String::new()
	}
}

pub fn write_egg_block(egg_block: &Egg, dependencies: &Vec<File>) -> Result<Bytes, Box<dyn Error>> {

	let mut tag_dependencies: Vec<&File> = Vec::new();
	for dependency in dependencies {
		if egg_block.dependencies.contains(&dependency.filename()) {
			tag_dependencies.push(dependency);
		}
	}

	let mut int_values: Vec<(String, u32)> = Vec::new();
	let mut str_values: Vec<(String, String)> = Vec::new();

	int_values.push(("Agent Type".to_string(), 0));

	str_values.push(("Egg Gallery male".to_string(), file_stem(&egg_block.sprite_file_male)));
	str_values.push(("Egg Glyph File".to_string(), egg_block.sprite_file_male.clone()));

	str_values.push(("Egg Gallery female".to_string(), file_stem(&egg_block.sprite_file_female)));
	str_values.push(("Egg Glyph File 2".to_string(), egg_block.sprite_file_female.clone()));

	str_values.push(("Egg Animation String".to_string(), egg_block.animation_string.clone()));

	str_values.push(("Genetics File".to_string(), file_stem(&egg_block.genetics_file)));
	str_values.push(("Mother Genetic File".to_string(), file_stem(&egg_block.genetics_file_mother)));
	str_values.push(("Father Genetic File".to_string(), file_stem(&egg_block.genetics_file_father)));

	write_dependencies(&mut int_values, &mut str_values, &tag_dependencies);

	let content_buffer = write_tag_block(&int_values, &str_values);

	let block_header = BlockHeader {
		id: "EGGS".to_string(),
		name: egg_block.name.clone(),
		size_compressed: content_buffer.len(),
		size_uncompressed: content_buffer.len(),
		is_compressed: false
	};

	let mut buffer = BytesMut::new();
	buffer.extend_from_slice(&write_block_header(block_header));
	buffer.extend_from_slice(&content_buffer);
	Ok(buffer.freeze())
}
