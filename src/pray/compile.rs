use super::helper::write_string;
use super::file_block::write_file_block;
use crate::agent::file::CreaturesFile;
use crate::agent::tag::Tag;

use std::error::Error;
use bytes::{ Bytes, BytesMut };

pub fn compile(tags: &Vec<Box<dyn Tag>>, files: &mut Vec<CreaturesFile>) -> Result<Bytes, Box<dyn Error>> {
	let mut buffer = BytesMut::new();

	buffer.extend_from_slice(&write_string("PRAY", 4));

	for tag in tags {
		buffer.extend_from_slice(&tag.write_block(files)?);
	}

	for file in files {
		if &file.get_extension() != "cos" {
			buffer.extend_from_slice(&write_file_block(file)?);
		}
	}

	Ok(buffer.freeze())
}
