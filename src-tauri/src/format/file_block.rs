use std::error::Error;
use bytes::{ Bytes, BytesMut };

use super::pray::{
	BlockHeader,
	write_block_header,
	compress_block_contents,
};

#[derive(Clone, serde::Serialize)]
pub struct File {
	pub name: String,
	pub extension: String,
	pub data: Vec<u8>,
	pub is_checked: bool
}

impl File {
	pub fn filename(&self) -> String {
		format!("{}.{}", self.name, self.extension)
	}
}

pub fn write_file_block(file_block: &File) -> Result<Bytes, Box<dyn Error>> {
	let mut buffer = BytesMut::new();
	let compressed_data = compress_block_contents(&file_block.data)?;
	let block_header = BlockHeader {
		id: "FILE".to_string(),
		name: format!("{}.{}", file_block.name, file_block.extension),
		size_compressed: compressed_data.len(),
		size_uncompressed: file_block.data.len(),
		is_compressed: true
	};
	buffer.extend_from_slice(&write_block_header(block_header));
	buffer.extend_from_slice(&compressed_data);
	Ok(buffer.freeze())
}
