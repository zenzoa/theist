use crate::blk;
use crate::agent::*;

use std::fs;
use std::str;
use std::error::Error;
use image::io::Reader as ImageReader;
use bytes::Bytes;

#[derive(Clone)]
pub enum Background {
	Blk { filename: Filename },
	Png { filename: Filename, source: Filename }
}

impl Background {
	pub fn new(filename: &str) -> Background {
		let filename = Filename::new(filename);
		match filename.extension.as_str() {
			"blk" => Background::Blk { filename },
			_ => Background::Png {
				filename: filename.with_extension("blk"),
				source: filename
			}
		}
	}

	pub fn get_filename(&self) -> String {
		match self {
			Background::Blk { filename } => filename.to_string(),
			Background::Png { source, .. } => source.to_string()
		}
	}

	pub fn get_title(&self) -> String {
		match self {
			Background::Blk { filename } => filename.title.clone(),
			Background::Png { filename, .. } => filename.title.clone()
		}
	}

	pub fn get_data(&self, path: &str) -> Result<Bytes, Box<dyn Error>> {
		match self {
			Background::Blk { filename } => {
				let filepath = format!("{}{}", path, filename);
				let contents = fs::read(&filepath)?;
				println!("  Got data from {}", &filepath);
				Ok(Bytes::copy_from_slice(&contents))
			},
			Background::Png { source, .. } => {
				let filepath = format!("{}{}", path, source);
				let image_data = ImageReader::open(&filepath)?;
				let image = image_data.decode()?;
				println!("  Got data from {}", &filepath);
				Ok(Bytes::from(blk::encode(image.into_rgba8())))
			}
		}
	}

	pub fn convert_to_sprite(&self) -> Option<Sprite> {
		if let Background::Png{ source, .. } = self {
			let mut new_sprite = Sprite::new(format!("{}.c16", &source.title).as_str());
			new_sprite.add_frame(source.to_string().as_str());
			return Some(new_sprite);
		}
		None
	}
}

#[derive(Clone)]
pub struct BackgroundList(Vec<Background>);

impl BackgroundList {
	pub fn new() -> BackgroundList {
		BackgroundList(Vec::new())
	}

	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	pub fn len(&self) -> usize {
		self.0.len()
	}

	pub fn iter(&self) -> std::slice::Iter<'_, Background> {
		self.0.iter()
	}

	pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Background> {
		self.0.iter_mut()
	}

	pub fn get(&self, index: usize) -> Option<&Background> {
		self.0.get(index)
	}

	pub fn get_mut(&mut self, index: usize) -> Option<&mut Background> {
		self.0.get_mut(index)
	}

	pub fn push(&mut self, background: Background) {
		self.0.push(background)
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
