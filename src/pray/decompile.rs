use super::helper::{ BlockHeader, read_string, read_block_header };
use super::agent_block::read_agent_block;
use super::egg_block::read_egg_block;
use super::file_block::read_file_block;
use super::free_block::read_free_block;
use crate::agent::agent_tag::SupportedGame;
use crate::agent::file::CreaturesFile;
use crate::agent::tag::Tag;
use crate::error::create_error;

use std::io::Read;
use std::error::Error;
use bytes::{ Bytes, Buf };
use libflate::zlib;

pub struct DecompileResult {
	pub tags: Vec<Tag>,
	pub files: Vec<CreaturesFile>
}

pub fn decompile(contents: &[u8], convert_sprites_to_pngs: bool) -> Result<DecompileResult, Box<dyn Error>> {
	let mut tags: Vec<Tag> = Vec::new();
	let mut files: Vec<CreaturesFile> = Vec::new();

	// first read all file blocks
	let mut buffer_for_files = Bytes::copy_from_slice(contents);
	if buffer_for_files.len() >= 4 && read_string(&mut buffer_for_files, 4) == "PRAY" {
		while buffer_for_files.len() >= 144 {
			let block_header = read_block_header(&mut buffer_for_files);
			if buffer_for_files.len() >= block_header.size {
				let mut block_contents = get_block_contents(&mut buffer_for_files, &block_header)?;
				if block_header.block_type.as_str() == "FILE" {
					files.append(
						&mut read_file_block(&mut block_contents, &block_header.name, convert_sprites_to_pngs)?
					);
				}
			}
		}
	}

	// then read all remaining blocks
	let mut buffer = Bytes::copy_from_slice(contents);
	if buffer.len() >= 4 {
		if read_string(&mut buffer, 4) == "PRAY" {
			while buffer.len() >= 144 {
				let block_header = read_block_header(&mut buffer);

				if buffer.len() >= block_header.size {
					let mut block_contents = get_block_contents(&mut buffer, &block_header)?;

					match block_header.block_type.as_str() {
						"AGNT" => {
							let tag = read_agent_block(&mut block_contents, &block_header.name, SupportedGame::C3, &mut files);
							tags.push(tag);
						},
						"DSAG" => {
							let tag = read_agent_block(&mut block_contents, &block_header.name, SupportedGame::DS, &mut files);
							tags.push(tag);
						},
						"EGGS" => {
							tags.push(
								read_egg_block(&mut block_contents, &block_header.name, &files)
							);
						},
						"FILE" => (),
						_ => {
							tags.push(
								read_free_block(&mut block_contents, &block_header.block_type, &block_header.name)?
							);
						}
					}
				} else {
					return Err(create_error(format!("File ends before block {} does.", block_header.name).as_str()));
				}
			}
		} else {
			return Err(create_error("File is missing valid header."));
		}
	}

	Ok(DecompileResult{ tags, files })
}

fn get_block_contents(buffer: &mut Bytes, block_header: &BlockHeader) -> Result<Bytes, Box<dyn Error>> {
	let mut block_contents = buffer.copy_to_bytes(block_header.size);
	if block_header.is_compressed {
		let mut decoder = zlib::Decoder::new(block_contents.chunk())?;
		let mut decoded_data = Vec::new();
		decoder.read_to_end(&mut decoded_data)?;
		block_contents = Bytes::from(decoded_data);
	}
	Ok(block_contents)
}
