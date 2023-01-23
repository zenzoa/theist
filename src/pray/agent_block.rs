use super::info_block::{ IntValue, StrValue, InfoValue, write_info_block, read_info_block };
use super::helper::write_block_header;
use crate::agent::tag::Tag;
use crate::agent::script::Script;
use crate::agent::agent_tag::{ AgentTag, SupportedGame, Preview, RemoveScript };
use crate::agent::file::CreaturesFile;
use crate::error::create_error;
use crate::file_helper;

use std::str;
use std::error::Error;
use bytes::{ Bytes, BytesMut };
use regex::Regex;

pub fn write_agent_block(tag: &AgentTag, files: &[CreaturesFile]) -> Result<Bytes, Box<dyn Error>> {
	let mut int_values: Vec<IntValue> = Vec::new();
	let mut str_values: Vec<StrValue> = Vec::new();

	int_values.push(IntValue("Agent Type".to_string(), 0));

	if !tag.description.is_empty() {
		str_values.push(StrValue("Agent Description".to_string(), tag.description.to_string()));
	}

	match &tag.preview {
		Preview::Manual{ sprite, animation } => {
			str_values.push(StrValue("Agent Animation File".to_string(), format!("{}.c16", sprite)));
			str_values.push(StrValue("Agent Animation Gallery".to_string(), sprite.to_string()));
			str_values.push(StrValue("Agent Animation String".to_string(), animation.to_string()));
		},
		Preview::Auto => {
			if let Some(sprite) = tag.sprites.get(0) {
				str_values.push(StrValue("Agent Animation File".to_string(), format!("{}.c16", sprite)));
				str_values.push(StrValue("Agent Animation Gallery".to_string(), sprite.to_string()));
				str_values.push(StrValue("Agent Animation String".to_string(), "0".to_string()));
			}
		},
		_ => {}
	}

	let mut first_script = "".to_string();
	int_values.push(IntValue("Script Count".to_string(), tag.scripts.len() as u32));
	for (i, script) in tag.scripts.iter().enumerate() {
		for file in files {
			if &file.get_output_filename() == script {
				match file.get_data() {
					Some(script_data) => {
						let script_string = str::from_utf8(&script_data)?;
						if first_script.is_empty() { first_script = script_string.to_string(); }
						str_values.push(StrValue(format!("Script {}", i + 1), script_string.to_string()));
					},
					None => {
						return Err(create_error(format!("Unable to find data for script {}", &script).as_str()));
					}
				}
				break;
			}
		}
	}

	match &tag.remove_script {
		RemoveScript::Manual(remove_script) => {
			if !remove_script.is_empty() {
				str_values.push(StrValue("Remove script".to_string(), remove_script.to_string()));
			}
		},
		RemoveScript::Auto => {
			if !first_script.is_empty() {
				let remove_script_pattern = Regex::new(r"[\n\r]rscr[\n\r]([\s\S]*)").unwrap();
				if let Some(captures) = remove_script_pattern.captures(&first_script) {
					if let Some(remove_script) = captures.get(1) {
						let remove_newlines_pattern = Regex::new(r"\s+").unwrap();
						let remove_script = remove_newlines_pattern.replace_all(remove_script.as_str(), " ").trim().to_string();
						str_values.push(StrValue("Remove script".to_string(), remove_script));
					}
				}
			}
		},
		_ => {}
	}

	let mut dependency_count = 0;

	for sprite in &tag.sprites {
		dependency_count += 1;
		int_values.push(IntValue(format!("Dependency Category {}", dependency_count), 2));
		str_values.push(StrValue(format!("Dependency {}", dependency_count), sprite.to_string()));
	}

	for sound in &tag.sounds {
		dependency_count += 1;
		int_values.push(IntValue(format!("Dependency Category {}", dependency_count), 1));
		str_values.push(StrValue(format!("Dependency {}", dependency_count), sound.to_string()));
	}

	for catalogue in &tag.catalogues {
		dependency_count += 1;
		int_values.push(IntValue(format!("Dependency Category {}", dependency_count), 7));
		str_values.push(StrValue(format!("Dependency {}", dependency_count), catalogue.to_string()));
	}

	int_values.push(IntValue("Dependency Count".to_string(), dependency_count));

	let blocks_buffer = write_info_block(int_values, str_values);
	let blocks_size = blocks_buffer.len();

	let mut buffer = BytesMut::new();
	let block_type = match tag.supported_game {
		SupportedGame::C3 => "AGNT",
		_ => "DSAG"
	};
	buffer.extend_from_slice(&write_block_header(block_type, &tag.name, blocks_size as u32));
	buffer.extend_from_slice(&blocks_buffer);

	Ok(buffer.freeze())
}

pub fn read_agent_block(contents: &mut Bytes, block_name: &String, supported_game: SupportedGame) ->
	(Box<dyn Tag>, Vec<CreaturesFile>)
{
	let mut description = String::new();
	let mut preview_sprite = String::new();
	let mut preview_animation = String::new();
	let mut remove_script = RemoveScript::None;

	let mut scripts: Vec<String> = Vec::new();
	let mut sprites: Vec<String> = Vec::new();
	let mut sounds: Vec<String> = Vec::new();
	let mut catalogues: Vec<String> = Vec::new();

	let mut files: Vec<CreaturesFile> = Vec::new();

	let info = read_info_block(contents);
	for (key, value) in info {
		match key.as_str() {
			"Agent Description" => if let InfoValue::Str(value) = value {
				description = value.clone();
			},
			"Agent Animation Gallery" => if let InfoValue::Str(value) = value {
				preview_sprite = value.clone();
			},
			"Agent Animation String" => if let InfoValue::Str(value) = value {
				preview_animation = value.clone();
			},
			"Remove script" => if let InfoValue::Str(value) = value {
				remove_script = RemoveScript::Manual(value.clone());
			},
			_ => {
				if key.starts_with("Script") {
					if let InfoValue::Str(value) = value {
						let filename = format!("{}.cos", block_name);
						scripts.push(filename.clone());
						files.push(CreaturesFile::Script(Script{
							input_filename: filename.clone(),
							output_filename: filename.clone(),
							data: Some(Bytes::from(value))
						}))
					}
				} else if key.starts_with("Dependency") {
					if let InfoValue::Str(value) = value {
						match file_helper::extension(&value).as_str() {
							"c16" => sprites.push(value.clone()),
							"s16" => sprites.push(value.clone()),
							"blk" => sprites.push(value.clone()),
							"wav" => sounds.push(value.clone()),
							"catalogue" => catalogues.push(value.clone()),
							_ => ()
						}
					}
				}
			}
		}
	}

	let preview = if preview_sprite.is_empty() || preview_animation.is_empty() {
		if sprites.is_empty() {
			Preview::None
		} else {
			Preview::Auto
		}
	} else {
		Preview::Manual {
			sprite: preview_sprite,
			animation: preview_animation
		}
	};

	(Box::new(AgentTag {
		name: block_name.to_string(),
		version: "".to_string(),

		description,
		supported_game,
		remove_script,
		preview,

		scripts,
		sprites,
		sounds,
		catalogues,

		use_all_files: false
	}), files)
}
