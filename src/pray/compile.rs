use super::helper::write_string;
use super::file_block::write_file_block;
use crate::agent::file::{ CreaturesFile, only_used_files };
use crate::agent::tag::Tag;

use std::error::Error;
use bytes::{ Bytes, BytesMut };

pub fn compile(tags: &Vec<Tag>, files: &mut [CreaturesFile]) -> Result<Bytes, Box<dyn Error>> {
	let mut buffer = BytesMut::new();

	buffer.extend_from_slice(&write_string("PRAY", 4));

	for tag in tags {
		buffer.extend_from_slice(&tag.write_block(files)?);
	}

	let mut files = only_used_files(tags, files);
	files.sort_by_key(|f| format!("{} {}", f.get_category_id(), f.get_output_filename()));

	for file in &mut files {
		if &file.get_extension() != "cos" {
			buffer.extend_from_slice(&write_file_block(file)?);
		}
	}

	Ok(buffer.freeze())
}
