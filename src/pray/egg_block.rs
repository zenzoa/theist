use super::info_block::{ IntValue, StrValue, InfoValue, write_info_block, read_info_block };
use super::helper::write_block_header;
use crate::agent::tag::Tag;
use crate::agent::egg_tag::EggTag;
use crate::agent::file::CreaturesFile;
use crate::file_helper;

use std::error::Error;
use bytes::{ Bytes, BytesMut };

pub fn write_egg_block(tag: &EggTag, files: &[CreaturesFile]) -> Result<Bytes, Box<dyn Error>> {
	let mut int_values: Vec<IntValue> = Vec::new();
	let mut str_values: Vec<StrValue> = Vec::new();

	int_values.push(IntValue("Agent Type".to_string(), 0));

	if !tag.preview_sprite_male.is_empty() {
		str_values.push(StrValue("Egg Gallery male".to_string(), tag.preview_sprite_male.to_string()));
		str_values.push(StrValue("Egg Glyph File".to_string(), format!("{}.c16", &tag.preview_sprite_male)));
	}

	if !tag.preview_sprite_female.is_empty() {
		str_values.push(StrValue("Egg Gallery female".to_string(), tag.preview_sprite_female.to_string()));
		str_values.push(StrValue("Egg Glyph File 2".to_string(), format!("{}.c16", &tag.preview_sprite_female)));
	}

	if !tag.preview_animation.is_empty() {
		str_values.push(StrValue("Egg Animation String".to_string(), tag.preview_animation.to_string()));
	}

	let mut dependency_count = 0;

	if !tag.genome.is_empty() {
		for file in files {
			let filename = file.get_title();
			let extension = file.get_extension();
			if filename == tag.genome && extension == "gen" {
				str_values.push(StrValue("Genetics File".to_string(), format!("{}*", &tag.genome)));
			}
			if filename == tag.genome && (extension == "gen" || extension == "gno") {
				dependency_count += 1;
				int_values.push(IntValue(format!("Dependency Category {}", dependency_count), 3));
				str_values.push(StrValue(format!("Dependency {}", dependency_count), format!("{}.{}", filename, extension)));
			}
		}
	}

	for sprite in &tag.sprites {
		dependency_count += 1;
		int_values.push(IntValue(format!("Dependency Category {}", dependency_count), 2));
		str_values.push(StrValue(format!("Dependency {}", dependency_count), sprite.to_string()));
	}


	for bodydata in &tag.bodydata {
		dependency_count += 1;
		int_values.push(IntValue(format!("Dependency Category {}", dependency_count), 4));
		str_values.push(StrValue(format!("Dependency {}", dependency_count), bodydata.to_string()));
	}

	int_values.push(IntValue("Dependency Count".to_string(), dependency_count));

	let blocks_buffer = write_info_block(int_values, str_values);
	let blocks_size = blocks_buffer.len();

	let mut buffer = BytesMut::new();
	buffer.extend_from_slice(&write_block_header("EGGS", &tag.name, blocks_size as u32));
	buffer.extend_from_slice(&blocks_buffer);

	Ok(buffer.freeze())
}

pub fn read_egg_block(contents: &mut Bytes, block_name: &String) -> Box<dyn Tag> {
	let mut preview_sprite_male = String::new();
	let mut preview_sprite_female = String::new();
	let mut preview_animation = String::new();
	let mut genome = String::new();
	let mut sprites: Vec<String> = Vec::new();
	let mut bodydata: Vec<String> = Vec::new();

	let info = read_info_block(contents);
	for (key, value) in info {
		match key.as_str() {
			"Egg Gallery female" => {
				if let InfoValue::Str(value) = value {
					preview_sprite_female = value.clone();
				}
			},
			"Egg Gallery male" => {
				if let InfoValue::Str(value) = value {
					preview_sprite_male = value.clone();
				}
			},
			"Egg Animation String" => {
				if let InfoValue::Str(value) = value {
					preview_animation = value.clone();
				}
			},
			"Genetics File" => {
				if let InfoValue::Str(value) = value {
					genome = value.replace('*', "");
				}
			},
			_ => {
				if key.starts_with("Dependency") {
					if let InfoValue::Str(value) = value {
						match file_helper::extension(&value).as_str() {
							"c16" => sprites.push(value.clone()),
							"att" => bodydata.push(value.clone()),
							_ => ()
						}
					}
				}
			}
		}
	}

	Box::new(EggTag {
		name: block_name.to_string(),
		version: "".to_string(),

		preview_sprite_male,
		preview_sprite_female,
		preview_animation,
		genome,

		sprites,
		bodydata,

		use_all_files: false
	})
}
