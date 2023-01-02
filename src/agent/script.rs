use crate::agent::*;

use std::fs;
use std::io;
use std::str;
use bytes::Bytes;

#[derive(Clone)]
pub enum Script {
	File { filename: Filename, supported_game: SupportedGame },
	//Inline { contents: String, supported_game: SupportedGame }
}

impl Script {
	pub fn new(filename: &str, supported_game: &str) -> Script {
		Script::File {
			filename: Filename::new(filename),
			supported_game: SupportedGame::new(supported_game)
		}
	}

	pub fn get_filename(&self) -> String {
		match self {
			Script::File{ filename, .. } => filename.to_string()
		}
	}

	pub fn get_data(&self, path: &str) -> Result<Bytes, io::Error> {
		match self {
			Script::File { filename, .. } => {
				let filepath = format!("{}{}", path, filename);
				let contents = fs::read(&filepath)?;
				println!("  Got data from {}", &filepath);
				Ok(Bytes::copy_from_slice(&contents))
			}
		}
	}

	pub fn set_supported_game(&mut self, new_supported_game: usize) {
		let Script::File{ supported_game, .. } = self;
		*supported_game = match new_supported_game {
			1 => SupportedGame::C3,
			2 => SupportedGame::DS,
			_ => SupportedGame::C3DS
		};
	}
}

#[derive(Clone)]
pub struct ScriptList(Vec<Script>);

impl ScriptList {
	pub fn new() -> ScriptList {
		ScriptList(Vec::new())
	}

	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	pub fn len(&self) -> usize {
		self.0.len()
	}

	pub fn includes(&self, filename: &String) -> bool {
		for x in &self.0 {
			if x.get_filename() == *filename {
				return true;
			}
		}
		false
	}

	pub fn iter(&self) -> std::slice::Iter<'_, Script> {
		self.0.iter()
	}

	pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Script> {
		self.0.iter_mut()
	}

	pub fn get(&self, index: usize) -> Option<&Script> {
		self.0.get(index)
	}

	pub fn get_mut(&mut self, index: usize) -> Option<&mut Script> {
		self.0.get_mut(index)
	}

	pub fn push(&mut self, script: Script) {
		self.0.push(script)
	}

	pub fn remove(&mut self, index: usize) {
		if index < self.0.len() {
			self.0.remove(index);
		}
	}

	pub fn move_up(&mut self, index: usize) {
		if index > 0 && index < self.0.len() {
			self.0.swap(index, index - 1);
		}
	}

	pub fn move_down(&mut self, index: usize) {
		if index + 1 < self.0.len() {
			self.0.swap(index, index + 1);
		}
	}
}
