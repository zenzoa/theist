use crate::agent::*;

use std::fs;
use std::str;
use std::error::Error;
use bytes::Bytes;

#[derive(Clone)]
pub struct Sound {
	pub filename: Filename
}

impl Sound {
	pub fn new(filename: &str) -> Sound {
		Sound {
			filename: Filename::new(filename)
		}
	}

	pub fn get_filename(&self) -> String {
		self.filename.to_string()
	}

	pub fn get_data(&self, path: &str) -> Result<Bytes, Box<dyn Error>> {
		let filepath = format!("{}{}", path, self.filename);
		let contents = fs::read(&filepath)?;
		println!("  Got data from {}", &filepath);
		Ok(Bytes::copy_from_slice(&contents))
	}
}

#[derive(Clone)]
pub struct SoundList(Vec<Sound>);

impl SoundList {
	pub fn new() -> SoundList {
		SoundList(Vec::new())
	}

	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	pub fn len(&self) -> usize {
		self.0.len()
	}

	pub fn iter(&self) -> std::slice::Iter<'_, Sound> {
		self.0.iter()
	}

	pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Sound> {
		self.0.iter_mut()
	}

	pub fn get(&self, index: usize) -> Option<&Sound> {
		self.0.get(index)
	}

	pub fn get_mut(&mut self, index: usize) -> Option<&mut Sound> {
		self.0.get_mut(index)
	}

	pub fn push(&mut self, sound: Sound) {
		self.0.push(sound)
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
