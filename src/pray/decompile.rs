use super::helper::{ read_string, read_block_header };
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
	pub tags: Vec<Box<dyn Tag>>,
	pub files: Vec<CreaturesFile>
}

pub fn decompile(contents: &[u8], convert_sprites_to_pngs: bool) -> Result<DecompileResult, Box<dyn Error>> {
	// look at each block
	// convert tag blocks into tags
	// convert file blocks into file data
	// write theist source file
	// TODO: when opening agent file, mark as unsaved; when saving, ask whether to extract inline files as well
	// TODO: in the UI, mark files that are not yet extracted, these are read-only (they have data that isn't empty - so empty this out when extracted)
	let mut tags: Vec<Box<dyn Tag>> = Vec::new();
	let mut files: Vec<CreaturesFile> = Vec::new();

	let mut buffer = Bytes::copy_from_slice(contents);

	if buffer.len() >= 4 {
		if read_string(&mut buffer, 4) == "PRAY" {
			while buffer.len() >= 144 {
				let block_header = read_block_header(&mut buffer);

				if buffer.len() >= block_header.size {
					let mut block_contents = buffer.copy_to_bytes(block_header.size);

					if block_header.is_compressed {
						let mut decoder = zlib::Decoder::new(block_contents.chunk())?;
						let mut decoded_data = Vec::new();
						decoder.read_to_end(&mut decoded_data)?;
						block_contents = Bytes::from(decoded_data);
					}

					match block_header.block_type.as_str() {
						"AGNT" => {
							let (tag, mut scripts) = read_agent_block(&mut block_contents, &block_header.name, SupportedGame::C3);
							tags.push(tag);
							files.append(&mut scripts);
						},
						"DSAG" => {
							let (tag, mut scripts) = read_agent_block(&mut block_contents, &block_header.name, SupportedGame::DS);
							tags.push(tag);
							files.append(&mut scripts);
						},
						"EGGS" => {
							tags.push(
								read_egg_block(&mut block_contents, &block_header.name)
							);
						},
						"FILE" => {
							files.append(
								&mut read_file_block(&mut block_contents, &block_header.name, convert_sprites_to_pngs)?
							);
						},
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
