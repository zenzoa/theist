use crate::agent::file::CreaturesFile;
use crate::agent::script::Script;
use crate::agent::sprite::Sprite;
use crate::agent::sound::Sound;
use crate::agent::bodydata::BodyData;
use crate::agent::catalogue::Catalogue;
use crate::agent::genetics::Genetics;
use crate::error::create_error;
use crate::file_helper;
use super::helper::write_block_header;

use std::error::Error;
use bytes::{ Bytes, BytesMut };

pub fn write_file_block(file: &mut CreaturesFile) -> Result<Bytes, Box<dyn Error>> {
	let mut buffer = BytesMut::new();
	let filename = file.get_output_filename();
	match file.get_data() {
		Some(data) => {
			buffer.extend_from_slice(&write_block_header("FILE", &filename, data.len() as u32));
			buffer.extend_from_slice(&data);
		},
		None => {
			return Err(create_error(format!("Unable to find data for file {}.", filename).as_str()));
		}
	}
	Ok(buffer.freeze())
}

pub fn read_file_block(contents: &mut Bytes, block_name: &String, convert_sprites_to_pngs: bool) -> Result<Vec<CreaturesFile>, Box<dyn Error>> {
	match file_helper::extension(block_name).as_str() {
		"cos" => {
			Ok(vec![ CreaturesFile::Script(Script::new_from_data(block_name, contents)?) ])
		},
		"c16" => {
			if convert_sprites_to_pngs {
				read_sprite(contents, block_name)
			} else {
				Ok(vec![ CreaturesFile::Sprite(Sprite::new_from_data(block_name, contents)?) ])
			}
		},
		"s16" => {
			if convert_sprites_to_pngs {
				read_sprite(contents, block_name)
			} else {
				Ok(vec![ CreaturesFile::Sprite(Sprite::new_from_data(block_name, contents)?) ])
			}
		},
		"blk" => {
			if convert_sprites_to_pngs {
				read_sprite(contents, block_name)
			} else {
				Ok(vec![ CreaturesFile::Sprite(Sprite::new_from_data(block_name, contents)?) ])
			}
		},
		"wav" => {
			Ok(vec![ CreaturesFile::Sound(Sound::new_from_data(block_name, contents)?) ])
		},
		"catalogue" => {
			Ok(vec![ CreaturesFile::Catalogue(Catalogue::new_from_data(block_name, contents)?) ])
		},
		"att" => {
			Ok(vec![ CreaturesFile::BodyData(BodyData::new_from_data(block_name, contents)?) ])
		},
		"gen" => {
			Ok(vec![ CreaturesFile::Genetics(Genetics::new_from_data(block_name, contents)?) ])
		},
		"gno" => {
			Ok(vec![ CreaturesFile::Genetics(Genetics::new_from_data(block_name, contents)?) ])
		},
		_ => {
			Err(create_error(format!("File block {} is not a valid creatures file type.", block_name).as_str()))
		}
	}
}

fn read_sprite(contents: &mut Bytes, block_name: &String) -> Result<Vec<CreaturesFile>, Box<dyn Error>> {
	let sprite = Sprite::new_from_data(block_name, contents)?;
	if let Sprite::Png{ frames, .. } = sprite {
		let mut frame_files: Vec<CreaturesFile> = Vec::new();
		for frame in frames {
			if let Some(frame_data) = &frame.data {
				frame_files.push(CreaturesFile::Sprite(Sprite::Raw{
					output_filename: file_helper::filename(&frame.input_filename),
					input_filename: frame.input_filename.to_string(),
					data: Some(frame_data.clone())
				}));
			}
		}
		Ok(frame_files)
	} else {
		Ok(Vec::new())
	}
}
