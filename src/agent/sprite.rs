use crate::c16;
use crate::agent::*;

use std::fs;
use std::str;
use std::error::Error;
use image::RgbaImage;
use image::io::Reader as ImageReader;
use bytes::Bytes;

#[derive(Clone)]
pub enum Sprite {
	C16 { filename: Filename },
	Frames { filename: Filename, frames: Vec<SpriteFrame> },
	//Spritesheet { filename: Filename, spritesheet_filename: Filename, cols: u32, rows: u32 }
}

impl Sprite {
	pub fn new(filename: &str) -> Sprite {
		Sprite::C16 {
			filename: Filename::new(filename)
		}
	}

	pub fn get_filename(&self) -> String {
		match self {
			Sprite::C16 { filename } => filename.to_string(),
			Sprite::Frames { filename, .. } => filename.to_string()
		}
	}

	pub fn get_title(&self) -> String {
		match self {
			Sprite::C16 { filename } => filename.title.clone(),
			Sprite::Frames { filename, .. } => filename.title.clone()
		}
	}

	pub fn get_data(&self, path: &str) -> Result<Bytes, Box<dyn Error>> {
		match self {
			Sprite::C16 { filename, .. } => {
				let filepath = format!("{}{}", path, filename);
				let contents = fs::read(&filepath)?;
				println!("  Got data from {}", &filepath);
				Ok(Bytes::copy_from_slice(&contents))
			},
			Sprite::Frames { frames, .. } => {
				let mut images: Vec<RgbaImage> = Vec::new();
				for frame in frames {
					let filepath = format!("{}{}", path, frame.filename);
					let image_data = ImageReader::open(&filepath)?;
					let image = image_data.decode()?;
					println!("  Got data from {}", &filepath);
					images.push(image.into_rgba8());
				}
				Ok(Bytes::from(c16::encode(images)))
			}
		}
	}

	pub fn set_name(&mut self, new_name: String) {
		if let Sprite::Frames{ filename, .. } = self {
			filename.set_title(new_name);
		}
	}

	pub fn convert_to_background(&self) -> Option<Background> {
		if let Sprite::Frames{ frames, .. } = self {
			if let Some(frame) = frames.get(0) {
				return Some(Background::new(frame.filename.to_string().as_str()));
			}
		}
		None
	}

	pub fn add_frame(&mut self, frame_filename: &str) {
		let frame = SpriteFrame::new(frame_filename);
		match self {
			Sprite::C16 { filename } => {
				*self = Sprite::Frames {
					filename: filename.clone(),
					frames: vec![ frame ]
				}
			},
			Sprite::Frames { frames, .. } => {
				frames.push(frame);
			}
		}
	}

	pub fn remove_frame(&mut self, index: usize) {
		if let Sprite::Frames{ frames, .. } = self {
			if index < frames.len() {
				frames.remove(index);
			}
		}
	}

	pub fn move_frame_up(&mut self, index: usize) {
		if let Sprite::Frames{ frames, .. } = self {
			if index > 0 && index < frames.len() {
				frames.swap(index, index - 1);
			}
		}
	}

	pub fn move_frame_down(&mut self, index: usize) {
		if let Sprite::Frames{ frames, .. } = self {
			if index + 1 < frames.len() {
				frames.swap(index, index + 1);
			}
		}
	}
}

#[derive(Clone)]
pub struct SpriteFrame {
	pub filename: Filename
}

impl SpriteFrame {
	pub fn new(filename: &str) -> SpriteFrame {
		SpriteFrame {
			filename: Filename::new(filename)
		}
	}
}

#[derive(Clone)]
pub struct SpriteList(Vec<Sprite>);

impl SpriteList {
	pub fn new() -> SpriteList {
		SpriteList(Vec::new())
	}

	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	pub fn len(&self) -> usize {
		self.0.len()
	}

	pub fn iter(&self) -> std::slice::Iter<'_, Sprite> {
		self.0.iter()
	}

	pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Sprite> {
		self.0.iter_mut()
	}

	pub fn get(&self, index: usize) -> Option<&Sprite> {
		self.0.get(index)
	}

	pub fn get_mut(&mut self, index: usize) -> Option<&mut Sprite> {
		self.0.get_mut(index)
	}

	pub fn push(&mut self, sprite: Sprite) {
		self.0.push(sprite)
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
