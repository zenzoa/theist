use std::error::Error;
use bytes::{ Bytes, BytesMut };

use super::pray::{
	BlockHeader,
	write_block_header,
};

#[derive(Clone, serde::Serialize)]
pub struct GenericBlock {
	pub id: String,
	pub name: String,
	pub data: Bytes
}

pub fn write_generic_block(generic_block: &GenericBlock) -> Result<Bytes, Box<dyn Error>> {
	let mut buffer = BytesMut::new();
	let block_header = BlockHeader {
		id: generic_block.id.clone(),
		name: generic_block.name.clone(),
		size_compressed: generic_block.data.len(),
		size_uncompressed: generic_block.data.len(),
		is_compressed: true
	};
	buffer.extend_from_slice(&write_block_header(block_header));
	buffer.extend_from_slice(&generic_block.data);
	Ok(buffer.freeze())
}
