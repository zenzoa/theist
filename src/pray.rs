use crate::c16;
use crate::blk;
use crate::agent::*;
use crate::agent::tag::*;
use crate::agent::agent_tag::*;
use crate::agent::script::*;
use crate::agent::sprite::*;
use crate::agent::background::*;
use crate::agent::sound::*;
use crate::agent::catalogue::*;

use std::str;
use std::io::Cursor;
use std::error::Error;
use std::collections::HashMap;
use bytes::{ Bytes, BytesMut, Buf, BufMut };
use image::{ ImageOutputFormat };

struct BlockHeader {
	block_type: String,
	name: String,
	size: usize,
	size_compressed: usize,
	is_compressed: bool
}

struct IntValue {
	name: String,
	value: u32
}

struct StrValue {
	name: String,
	value: String
}

enum InfoValue {
	Int(u32),
	Str(String)
}

fn read_string(buffer: &mut Bytes, num_bytes: usize) -> String {
	let mut string = String::from("");
	for _i in 0..num_bytes {
		let byte = buffer.get_u8();
		if byte != 0 {
			if let Ok(char) = str::from_utf8(&[byte]) {
				string += char;
			}
		}
	}
	string
}

fn read_info_block(buffer: &mut Bytes) -> HashMap<String, InfoValue> {
	let mut info: HashMap<String, InfoValue> = HashMap::new();

	let int_value_count = buffer.get_u32_le();
	for _i in 0..int_value_count {
		let name_length = buffer.get_u32_le();
		let name = read_string(buffer, name_length as usize);
		let value = buffer.get_u32_le();
		info.insert(name, InfoValue::Int(value));
	}

	let str_value_count = buffer.get_u32_le();
	for _i in 0..str_value_count {
		let name_length = buffer.get_u32_le();
		let name = read_string(buffer, name_length as usize);
		let value_length = buffer.get_u32_le();
		let value = read_string(buffer, value_length as usize);
		info.insert(name, InfoValue::Str(value));
	}

	info
}

fn read_block_header(buffer: &mut Bytes) -> BlockHeader {
	BlockHeader {
		block_type: read_string(buffer, 4),
		name: read_string(buffer, 128),
		size: buffer.get_u32_le() as usize,
		size_compressed: buffer.get_u32_le() as usize,
		is_compressed: buffer.get_u32_le() == 1
	}
}

fn read_agent_block(buffer: &mut Bytes, files: &mut Vec<FileData>, block_name: String, supported_game: SupportedGame) -> AgentTag {
	let mut tag = AgentTag::new(block_name.clone());
	tag.supported_game = supported_game;

	let mut preview_sprite = String::from("");
	let mut preview_animation = String::from("");

	let info = read_info_block(buffer);
	for (key, value) in info {
		match key.as_str() {
			"Agent Description" => {
				if let InfoValue::Str(value) = value {
					tag.description = value.clone();
				}
			},
			"Agent Animation Gallery" => {
				if let InfoValue::Str(value) = value {
					preview_sprite = value.clone();
				}
			},
			"Agent Animation String" => {
				if let InfoValue::Str(value) = value {
					preview_animation = value.clone();
				}
			},
			"Remove script" => {
				if let InfoValue::Str(value) = value {
					tag.removescript = RemoveScript::Manual(value.clone());
				}
			},
			_ => {
				if key.starts_with("Script") {
					if let InfoValue::Str(value) = value {
						let filename = format!("{}.cos", &block_name);
						let supported_game_string = format!("{}", &tag.supported_game);
						tag.scripts.push(Script::new(filename.as_str(), supported_game_string.as_str()));
						println!("Extracted file: {}", &filename);
						let data = Bytes::from(value);
						files.push(FileData {
							name: filename.clone(),
							data
						});
					}

				} else if key.starts_with("Dependency") {
					if let InfoValue::Str(value) = value {
						let filename = Filename::new(value.as_str());
						match filename.extension.as_str() {
							"c16" => tag.sprites.push(Sprite::Frames { filename, frames: Vec::new() }),
							"blk" => tag.backgrounds.push(Background::Blk {
								filename: Filename::new(format!("{}.png", &filename.title).as_str())
							}),
							"wav" => tag.sounds.push(Sound { filename }),
							"catalogue" => tag.catalogues.push(Catalogue::File { filename }),
							_ => ()
						}
					}
				}
			}
		}
	}

	if !preview_sprite.is_empty() && !preview_animation.is_empty() {
		tag.preview = Preview::Manual {
			sprite: preview_sprite,
			animation: preview_animation
		};
	}

	tag
}

pub fn decode(contents: &[u8]) -> Result<(Vec<Tag>, Vec<FileData>), Box<dyn Error>> {
	let mut tags: Vec<Tag> = Vec::new();
	let mut files: Vec<FileData> = Vec::new();

	let mut buffer = Bytes::copy_from_slice(contents);
	if buffer.len() >= 4 {
		let file_id = read_string(&mut buffer, 4);
		if file_id == "PRAY" {
			while buffer.len() >= 144 {
				let block_header = read_block_header(&mut buffer);
				if block_header.is_compressed {
					println!("ERROR: Unable to extract compressed data from block {} {}", block_header.block_type, block_header.name);
					if buffer.len() >= block_header.size_compressed {
						buffer.advance(block_header.size_compressed);
					} else {
						println!("ERROR: File ends before block {} {} ends", block_header.block_type, block_header.name);
						break;
					}
				} else if buffer.len() >= block_header.size {
					match block_header.block_type.as_str() {
						"AGNT" => {
							println!("Agent Block: {}", &block_header.name);
							let agent_tag = read_agent_block(&mut buffer, &mut files, block_header.name, SupportedGame::C3);
							tags.push(Tag::Agent(agent_tag));
						},
						"DSAG" => {
							println!("Agent Block: {}", &block_header.name);
							let agent_tag = read_agent_block(&mut buffer, &mut files, block_header.name, SupportedGame::DS);
							tags.push(Tag::Agent(agent_tag));
						},
						"FILE" => {
							let filename = Filename::new(block_header.name.as_str());
							let data = buffer.copy_to_bytes(block_header.size);
							match filename.extension.as_str() {
								"c16" => {
									let images = c16::decode(&data)?;
									for (i, image) in images.iter().enumerate() {
										let png_filename = format!("{}-{}.png", &filename.title, i + 1);
										for tag in &mut tags {
											if let Tag::Agent(tag) = tag {
												for sprite in tag.sprites.iter_mut() {
													if let Sprite::Frames { filename: sprite_filename, frames } = sprite {
														if sprite_filename.title.starts_with(&filename.title) {
															frames.push(SpriteFrame { filename: Filename::new(png_filename.as_str()) });
														}
													}
												}
											}
										}
										let mut png_data = Cursor::new(Vec::new());
										image.write_to(&mut png_data, ImageOutputFormat::Png)?;
										println!("Extracted file: {}", &png_filename);
										files.push(FileData {
											name: png_filename,
											data: Bytes::from(png_data.into_inner())
										});
									}
								},
								"blk" => {
									let image = blk::decode(&data);
									if let Ok(image) = image {
										let blk_filename = format!("{}.png", filename.title);
										let mut blk_data = Cursor::new(Vec::new());
										image.write_to(&mut blk_data, ImageOutputFormat::Png)?;
										println!("Extracted file: {}", &blk_filename);
										files.push(FileData {
											name: blk_filename,
											data: Bytes::from(blk_data.into_inner())
										});
									}
								},
								_ => {
									println!("Extracted file: {}", &filename);
									files.push(FileData {
										name: filename.to_string(),
										data
									});
								}
							}
						},
						_ => {
							println!("ERROR: Unknown block {} {}", block_header.block_type, block_header.name);
							buffer.advance(block_header.size);
						}
					}
				} else {
					println!("ERROR: File ends before block {} {} ends", block_header.block_type, block_header.name);
					break;
				}
			}
		}
	}

	Ok((tags, files))
}

fn write_string(buffer: &mut BytesMut, num_bytes: usize, string: &str) {
	for i in 0..num_bytes {
		if i >= string.len() {
			buffer.put_u8(0);
		} else {
			buffer.put_u8(*string.as_bytes().get(i).unwrap());
		}
	}
}

fn write_block_header(buffer: &mut BytesMut, block_type: &str, block_name: &String, block_size: u32) {
	write_string(buffer, 4, block_type);
	write_string(buffer, 128, format!("{}\0", block_name).as_str());
	buffer.put_u32_le(block_size); // uncompressed size
	buffer.put_u32_le(block_size); // compressed size
	buffer.put_u32_le(0); // compression flag - it's off, we're not compressing anything
}

fn write_info_block(buffer: &mut BytesMut, int_values: Vec<IntValue>, str_values: Vec<StrValue>) {
	buffer.put_u32_le(int_values.len() as u32);
	for val in int_values {
		buffer.put_u32_le(val.name.len() as u32);
		write_string(buffer, val.name.len(), val.name.as_str());
		buffer.put_u32_le(val.value);
	}

	buffer.put_u32_le(str_values.len() as u32);
	for val in str_values {
		buffer.put_u32_le(val.name.len() as u32);
		write_string(buffer, val.name.len(), val.name.as_str());
		buffer.put_u32_le(val.value.len() as u32);
		write_string(buffer, val.value.len(), val.value.as_str());
	}
}

fn write_file_block(buffer: &mut BytesMut, filename: &String, data: &Bytes) {
	println!("Write file block for \"{}\"", filename);
	write_block_header(buffer, "FILE", filename, data.len() as u32);
	buffer.extend_from_slice(data);
}

fn write_agent_block(buffer: &mut BytesMut, tag: &AgentTag) {
	println!("Write agent block for \"{}\"", tag.name);

	let block_type = match tag.supported_game {
		SupportedGame::C3 => "AGNT",
		_ => "DSAG"
	};

	let mut int_values: Vec<IntValue> = Vec::new();
	let mut str_values: Vec<StrValue> = Vec::new();

	int_values.push(IntValue{
		name: String::from("Agent Type"),
		value: 0
	});

	if !tag.description.is_empty() {
		str_values.push(StrValue{
			name: String::from("Agent Description"),
			value: tag.description.to_string()
		});
	}

	if let Preview::Manual { sprite, animation } = &tag.preview {
		println!("  Write injector preview");
		str_values.push(StrValue{
			name: String::from("Agent Animation File"),
			value: format!("{}.c16", sprite)
		});
		str_values.push(StrValue{
			name: String::from("Agent Animation Gallery"),
			value: sprite.to_string()
		});
		str_values.push(StrValue{
			name: String::from("Agent Animation String"),
			value: animation.to_string()
		});
	}

	println!("  Write {} scripts", tag.scripts.len());
	int_values.push(IntValue{
		name: String::from("Script Count"),
		value: tag.scripts.len() as u32
	});
	for i in 0..tag.scripts.len() {
		str_values.push(StrValue{
			name: format!("Script {}", i + 1),
			value: str::from_utf8(&tag.script_files[0]).unwrap().to_string()
		});
	}

	if let RemoveScript::Manual(removescript) = &tag.removescript {
		println!("  Write remove script");
		str_values.push(StrValue{
			name: String::from("Remove script"),
			value: removescript.to_string()
		});
	}

	let mut dependency_count = 0;

	println!("  Write sprite dependencies");
	for sprite in tag.sprites.iter() {
		dependency_count += 1;
		int_values.push(IntValue{
			name: format!("Dependency Category {}", dependency_count),
			value: 2
		});
		str_values.push(StrValue{
			name: format!("Dependency {}", dependency_count),
			value: sprite.get_filename()
		});
	}

	println!("  Write background dependencies");
	for background in tag.backgrounds.iter() {
		dependency_count += 1;
		int_values.push(IntValue{
			name: format!("Dependency Category {}", dependency_count),
			value: 2
		});
		str_values.push(StrValue{
			name: format!("Dependency {}", dependency_count),
			value: background.get_filename()
		});
	}

	println!("  Write sound dependencies");
	for sound in tag.sounds.iter() {
		dependency_count += 1;
		int_values.push(IntValue{
			name: format!("Dependency Category {}", dependency_count),
			value: 1
		});
		str_values.push(StrValue{
			name: format!("Dependency {}", dependency_count),
			value: sound.get_filename()
		});
	}

	println!("  Write catalogue dependencies");
	for catalogue in tag.catalogues.iter() {
		dependency_count += 1;
		int_values.push(IntValue{
			name: format!("Dependency Category {}", dependency_count),
			value: 7
		});
		str_values.push(StrValue{
			name: format!("Dependency {}", dependency_count),
			value: catalogue.get_filename()
		});
	}

	int_values.push(IntValue{
		name: String::from("Dependency Count"),
		value: dependency_count
	});

	let mut block_buffer = BytesMut::new();
	write_info_block(&mut block_buffer, int_values, str_values);

	write_block_header(buffer, block_type, &tag.name, block_buffer.len() as u32);
	buffer.unsplit(block_buffer);
}

pub fn encode(tags: &Vec<Tag>) -> Bytes {
	let mut buffer = BytesMut::new();

	let mut files_buffer = BytesMut::new();
	let mut files_written: Vec<String> = Vec::new();

	write_string(&mut buffer, 4, "PRAY");

	for tag in tags {
		if let Tag::Agent(tag) = tag {
			// agent info
			write_agent_block(&mut buffer, tag);

			// sprite files
			for (i, data) in tag.sprite_files.iter().enumerate() {
				let filename = tag.sprites.get(i).unwrap().get_filename();
				if !files_written.contains(&filename) {
					write_file_block(&mut files_buffer, &filename, data);
					files_written.push(filename);
				}
			}

			// background files
			for (i, data) in tag.background_files.iter().enumerate() {
				let filename = tag.backgrounds.get(i).unwrap().get_filename();
				if !files_written.contains(&filename) {
					write_file_block(&mut files_buffer, &filename, data);
					files_written.push(filename);
				}
			}

			// sound files
			for (i, data) in tag.sound_files.iter().enumerate() {
				let filename = tag.sounds.get(i).unwrap().get_filename();
				if !files_written.contains(&filename) {
					write_file_block(&mut files_buffer, &filename, data);
					files_written.push(filename);
				}
			}

			// catalogue files
			for (i, data) in tag.catalogue_files.iter().enumerate() {
				let filename = tag.catalogues.get(i).unwrap().get_filename();
				if !files_written.contains(&filename) {
					write_file_block(&mut files_buffer, &filename, data);
					files_written.push(filename);
				}
			}
		}
	}

	buffer.unsplit(files_buffer);
	Bytes::copy_from_slice(&buffer)
}
