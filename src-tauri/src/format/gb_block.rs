use std::{ str, error::Error };
use bytes::{ Bytes, BytesMut };

use super::file_block::File;
use super::agent_block::get_scripts;
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
pub struct GardenBox {
	pub name: String,
	pub description: String,
	pub author: String,
	pub category: u32, // 1: Patch Plant, 2: Traditional Plant, 3: Animal, 4: Aquatic Plant, 5: Aquatic Animal, 6: Decoration, 7: Tools, 8: Misc/Other
	pub animation_file: String,
	pub sprite_first_image: u32,
	pub remove_script: String,
	pub dependencies: Vec<String>,
}

pub fn read_gb_block(buffer: &mut Bytes, name: &str) -> Result<Vec<Block>, Box<dyn Error>> {
	let tag = read_tag_block(buffer)?;
	let mut agent_blocks: Vec<Block> = Vec::new();

	let mut dependencies = get_dependencies(&tag);

	for script_block in get_scripts(&tag, name) {
		if let Block::File(script) = &script_block {
			dependencies.push(format!("{}.cos", script.name));
		}
		agent_blocks.push(script_block);
	}

	agent_blocks.push(Block::GardenBox(GardenBox {
		name: name.to_string(),
		description: tag.str_values.get("Agent Description").unwrap_or(&String::new()).clone(),
		author: tag.str_values.get("Agent Author").unwrap_or(&String::new()).clone(),
		category: *tag.int_values.get("GB_Category").unwrap_or(&0) as u32,
		animation_file: tag.str_values.get("Agent Animation File").unwrap_or(&String::new()).clone(),
		sprite_first_image: *tag.int_values.get("Agent Sprite First Image").unwrap_or(&0) as u32,
		remove_script: tag.str_values.get("Remove script").unwrap_or(&String::new()).clone(),
		dependencies
	}));

	Ok(agent_blocks)
}

pub fn write_gb_block(gb_block: &GardenBox, dependencies: &Vec<File>) -> Result<Bytes, Box<dyn Error>> {
	let mut tag_scripts: Vec<&File> = Vec::new();
	let mut tag_dependencies: Vec<&File> = Vec::new();
	for dependency in dependencies {
		if gb_block.dependencies.contains(&dependency.filename()) {
			if dependency.extension == "cos" {
				tag_scripts.push(dependency);
			} else {
				tag_dependencies.push(dependency);
			}
		}
	}

	let mut int_values: Vec<(String, u32)> = Vec::new();
	let mut str_values: Vec<(String, String)> = Vec::new();

	int_values.push(("Agent Type".to_string(), 0));

	str_values.push(("Agent Description".to_string(), gb_block.description.clone()));
	str_values.push(("Agent Author".to_string(), gb_block.author.clone()));
	int_values.push(("GB_Category".to_string(), gb_block.category));

	if file_stem(&gb_block.animation_file) != "" {
		str_values.push(("Agent Animation Gallery".to_string(), file_stem(&gb_block.animation_file)));
		str_values.push(("Agent Animation File".to_string(), gb_block.animation_file.clone()));
		int_values.push(("Agent Sprite First Image".to_string(), gb_block.sprite_first_image));
	}

	write_dependencies(&mut int_values, &mut str_values, &tag_dependencies);

	str_values.push(("Remove script".to_string(), gb_block.remove_script.clone()));

	int_values.push(("Script Count".to_string(), tag_scripts.len() as u32));
	for (i, script) in tag_scripts.iter().enumerate() {
		str_values.push((format!("Script {}", i + 1), str::from_utf8(&script.data)?.to_string()));
	}

	let content_buffer = write_tag_block(&int_values, &str_values);

	let block_header = BlockHeader {
		id: "DSGB".to_string(),
		name: gb_block.name.clone(),
		size_compressed: content_buffer.len(),
		size_uncompressed: content_buffer.len(),
		is_compressed: false
	};

	let mut buffer = BytesMut::new();
	buffer.extend_from_slice(&write_block_header(block_header));
	buffer.extend_from_slice(&content_buffer);
	Ok(buffer.freeze())
}
