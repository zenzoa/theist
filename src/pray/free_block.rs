use super::helper::write_block_header;
use crate::agent::tag::Tag;
use crate::agent::free_tag::FreeTag;
use crate::agent::file::CreaturesFile;

use std::str;
use std::error::Error;
use bytes::{ Bytes, BytesMut };

pub fn write_free_block(tag: &FreeTag, _files: &[CreaturesFile]) -> Result<Bytes, Box<dyn Error>> {
	let mut buffer = BytesMut::new();
	buffer.extend_from_slice(&write_block_header(&tag.block_type.to_uppercase(), &tag.name, tag.contents.len() as u32));
	buffer.extend_from_slice(tag.contents.as_bytes());
	Ok(buffer.freeze())
}

pub fn read_free_block(contents: &mut Bytes, block_type: &String, block_name: &String) -> Result<Tag, Box<dyn Error>> {
	Ok(Tag::Free(FreeTag {
		name: block_name.to_string(),
		version: "".to_string(),
		block_type: block_type.to_string(),
		contents: str::from_utf8(contents)?.to_string()
	}))
}
