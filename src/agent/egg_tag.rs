use super::tag::Tag;
use super::file::CreaturesFile;
use crate::pray::egg_block::write_egg_block;
use crate::source::egg_tag;

use std::error::Error;
use bytes::Bytes;

#[derive(Clone)]
pub struct EggTag {
	pub name: String,
	pub version: String,

	pub preview_sprite_male: String,
	pub preview_sprite_female: String,
	pub preview_animation: String,
	pub genome: String,

	pub sprites: Vec<String>,
	pub bodydata: Vec<String>,

	pub use_all_files: bool
}

impl Tag for EggTag {
	fn get_type(&self) -> String {
		"egg".to_string()
	}

	fn get_name(&self) -> String {
		self.name.clone()
	}

	fn does_use_all_files(&self) -> bool {
		self.use_all_files
	}

	fn add_files(&mut self, files: &[CreaturesFile]) {
		for file in files {
			match file.get_extension().as_str() {
				"c16" => self.sprites.push(file.get_output_filename()),
				"s16" => self.sprites.push(file.get_output_filename()),
				"att" => self.bodydata.push(file.get_output_filename()),
				_ => ()
			}
		}
	}

	fn write_block(&self, files: &[CreaturesFile]) -> Result<Bytes, Box<dyn Error>> {
		write_egg_block(self, files)
	}

	fn encode(&self) -> String {
		egg_tag::encode(self)
	}

	fn split(&self) -> Vec<Box<dyn Tag>> {
		vec![ Box::new(self.clone()) ]
	}
}
