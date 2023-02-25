use super::info_block::{ IntValue, StrValue, InfoValue, write_info_block, read_info_block };
use super::helper::write_block_header;
use crate::agent::tag::Tag;
use crate::agent::egg_tag::{ EggTag, EggPreview };
use crate::agent::file::{ CreaturesFile, FileType, lookup_file_index };

use std::error::Error;
use bytes::{ Bytes, BytesMut };

pub fn write_egg_block(tag: &EggTag, files: &[CreaturesFile]) -> Result<Bytes, Box<dyn Error>> {
	let mut int_values: Vec<IntValue> = Vec::new();
	let mut str_values: Vec<StrValue> = Vec::new();

	int_values.push(IntValue("Agent Type".to_string(), 0));

	if let EggPreview::Manual{ sprite_male, sprite_female, animation } = &tag.preview {
		if let Some(sprite_male_file) = files.get(*sprite_male) {
			let sprite_male_filename = sprite_male_file.get_output_filename();
			str_values.push(StrValue("Egg Gallery male".to_string(), sprite_male_filename.to_string()));
			str_values.push(StrValue("Egg Glyph File".to_string(), format!("{}.c16", &sprite_male_filename)));
		}
		if let Some(sprite_female_file) = files.get(*sprite_female) {
			let sprite_female_filename = sprite_female_file.get_output_filename();
			str_values.push(StrValue("Egg Gallery female".to_string(), sprite_female_filename.to_string()));
			str_values.push(StrValue("Egg Glyph File 2".to_string(), format!("{}.c16", &sprite_female_filename)));
		}
		if !animation.is_empty() {
			str_values.push(StrValue("Egg Animation String".to_string(), animation.to_string()));
		}
	}

	let mut dependency_count = 0;

	if let Some(genome) = tag.genome {
		if let Some(genome_file) = files.get(genome) {
			str_values.push(StrValue("Genetics File".to_string(), format!("{}*", genome_file.get_output_filename())));
		}
	}

	for sprite in &tag.sprites {
		if let Some(CreaturesFile::Sprite(sprite_file)) = files.get(*sprite) {
			dependency_count += 1;
			int_values.push(IntValue(format!("Dependency Category {}", dependency_count), 2));
			str_values.push(StrValue(format!("Dependency {}", dependency_count), sprite_file.get_output_filename()));
		}
	}

	for bodydata in &tag.bodydata {
		if let Some(CreaturesFile::BodyData(bodydata_file)) = files.get(*bodydata) {
			dependency_count += 1;
			int_values.push(IntValue(format!("Dependency Category {}", dependency_count), 4));
			str_values.push(StrValue(format!("Dependency {}", dependency_count), bodydata_file.get_output_filename()));
		}
	}

	for genetics in &tag.genetics {
		if let Some(CreaturesFile::Genetics(genetics_file)) = files.get(*genetics) {
			dependency_count += 1;
			int_values.push(IntValue(format!("Dependency Category {}", dependency_count), 3));
			str_values.push(StrValue(format!("Dependency {}", dependency_count), genetics_file.get_output_filename()));
		}
	}

	int_values.push(IntValue("Dependency Count".to_string(), dependency_count));

	let blocks_buffer = write_info_block(int_values, str_values);
	let blocks_size = blocks_buffer.len();

	let mut buffer = BytesMut::new();
	buffer.extend_from_slice(&write_block_header("EGGS", &tag.name, blocks_size as u32));
	buffer.extend_from_slice(&blocks_buffer);

	Ok(buffer.freeze())
}

pub fn read_egg_block(contents: &mut Bytes, block_name: &String, files: &[CreaturesFile]) -> Tag {
	let mut preview_sprite_male = String::new();
	let mut preview_sprite_female = String::new();
	let mut preview_animation = "0".to_string();
	let mut genome = None;
	let mut sprites: Vec<usize> = Vec::new();
	let mut bodydata: Vec<usize> = Vec::new();
	let mut genetics: Vec<usize> = Vec::new();

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
					genome = lookup_file_index(files, &value.replace('*', ""));
				}
			},
			_ => {
				if key.starts_with("Dependency") {
					if let InfoValue::Str(value) = value {
						if let Some(file_index) = lookup_file_index(files, &value) {
							match files[file_index].get_filetype() {
								FileType::Sprite => sprites.push(file_index),
								FileType::BodyData => bodydata.push(file_index),
								FileType::Genetics => genetics.push(file_index),
								_ => ()
							}
						}
					}
				}
			}
		}
	}

	let mut preview = EggPreview::None;
	if let Some(sprite_male) = lookup_file_index(files, &preview_sprite_male) {
		if let Some(sprite_female) = lookup_file_index(files, &preview_sprite_female) {
			preview = EggPreview::Manual{ sprite_male, sprite_female, animation: preview_animation }
		}
	}

	Tag::Egg(EggTag {
		name: block_name.to_string(),
		version: "".to_string(),

		preview,
		genome,

		preview_backup: EggPreview::None,

		sprites,
		bodydata,
		genetics,

		use_all_files: false
	})
}
