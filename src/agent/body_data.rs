use crate::agent::*;

use std::fs;
use std::str;
use std::error::Error;
use bytes::Bytes;

#[derive(Clone)]
pub struct BodyData {
	pub filename: Filename
}

impl BodyData {
	pub fn new(filename: &str) -> BodyData {
		BodyData {
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
pub struct BodyDataList(Vec<BodyData>);

impl BodyDataList {
	pub fn new() -> BodyDataList {
		BodyDataList(Vec::new())
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

	pub fn iter(&self) -> std::slice::Iter<'_, BodyData> {
		self.0.iter()
	}

	pub fn get(&self, index: usize) -> Option<&BodyData> {
		self.0.get(index)
	}

	pub fn push(&mut self, body_data: BodyData) {
		self.0.push(body_data)
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
