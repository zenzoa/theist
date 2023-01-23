use super::tag::Tag;
use super::file::CreaturesFile;
use crate::pray::agent_block::write_agent_block;
use crate::source::agent_tag;
use crate::file_helper;

use std::error::Error;
use bytes::Bytes;

#[derive(Clone)]
pub struct AgentTag {
	pub name: String,
	pub version: String,

	pub description: String,
	pub supported_game: SupportedGame,
	pub remove_script: RemoveScript,
	pub preview: Preview,

	pub scripts: Vec<String>,
	pub sprites: Vec<String>,
	pub sounds: Vec<String>,
	pub catalogues: Vec<String>,

	pub use_all_files: bool
}

impl Tag for AgentTag {
	fn get_type(&self) -> String {
		"agent".to_string()
	}

	fn get_name(&self) -> String {
		self.name.clone()
	}

	fn get_scripts(&self) -> Vec<String> {
		self.scripts.clone()
	}

	fn does_use_all_files(&self) -> bool {
		self.use_all_files
	}

	fn add_files(&mut self, files: &[CreaturesFile]) {
		for file in files {
			match file.get_extension().as_str() {
				"cos" => self.scripts.push(file.get_output_filename()),
				"c16" => self.sprites.push(file.get_output_filename()),
				"s16" => self.sprites.push(file.get_output_filename()),
				"blk" => self.sprites.push(file.get_output_filename()),
				"wav" => self.sounds.push(file.get_output_filename()),
				"catalogue" => self.catalogues.push(file.get_output_filename()),
				_ => ()
			}
		}
	}

	fn write_block(&self, files: &[CreaturesFile]) -> Result<Bytes, Box<dyn Error>> {
		write_agent_block(self, files)
	}

	fn encode(&self) -> String {
		agent_tag::encode(self)
	}

	fn split(&self) -> Vec<Box<dyn Tag>> {
		match self.supported_game {
			SupportedGame::C3DS => {
				let mut c3_tag = self.clone();
				c3_tag.name.push_str(" C3");
				c3_tag.supported_game = SupportedGame::C3;
				let mut c3_scripts = Vec::new();
				for script in &mut c3_tag.scripts {
					c3_scripts.push(format!("{} C3.cos", file_helper::title(script)));
				}
				c3_tag.scripts = c3_scripts;

				let mut ds_tag = self.clone();
				ds_tag.name.push_str(" DS");
				ds_tag.supported_game = SupportedGame::DS;
				let mut ds_scripts = Vec::new();
				for script in &mut ds_tag.scripts {
					ds_scripts.push(format!("{} DS.cos", file_helper::title(script)));
				}
				ds_tag.scripts = ds_scripts;

				vec![ Box::new(c3_tag), Box::new(ds_tag) ]
			},

			_ => {
				vec![ Box::new(self.clone()) ]
			}
		}
	}
}

#[derive(Clone)]
pub enum SupportedGame {
	C3,
	DS,
	C3DS
}

#[derive(Clone, PartialEq)]
pub enum RemoveScript {
	None,
	Auto,
	Manual(String)
}

#[derive(Clone, PartialEq)]
pub enum Preview {
	None,
	Auto,
	Manual { sprite: String, animation: String }
}
