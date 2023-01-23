use super::tag::Tag;
use super::file::CreaturesFile;
use crate::pray::free_block::write_free_block;
use crate::source::free_tag;

use std::error::Error;
use bytes::Bytes;

#[derive(Clone)]
pub struct FreeTag {
	pub name: String,
	pub version: String,
	pub block_type: String,
	pub contents: String
}

impl Tag for FreeTag {
	fn get_type(&self) -> String {
		self.block_type.to_string()
	}

	fn get_name(&self) -> String {
		self.name.clone()
	}

	fn write_block(&self, files: &[CreaturesFile]) -> Result<Bytes, Box<dyn Error>> {
		write_free_block(self, files)
	}

	fn encode(&self) -> String {
		free_tag::encode(self)
	}

	fn split(&self) -> Vec<Box<dyn Tag>> {
		vec![ Box::new(self.clone()) ]
	}
}
