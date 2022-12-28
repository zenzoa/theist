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
			filename: Filename::new(filename, "c16")
		}
	}

	pub fn add_frame(&mut self, frame: SpriteFrame) {
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

	pub fn get_filename(&self) -> String {
		match self {
			Sprite::C16 { filename } => filename.as_string(),
			Sprite::Frames { filename, .. } => filename.as_string()
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
}

#[derive(Clone)]
pub struct SpriteFrame {
	pub filename: Filename
}

impl SpriteFrame {
	pub fn new(filename: &str) -> SpriteFrame {
		SpriteFrame {
			filename: Filename::new(filename, "png")
		}
	}
}
