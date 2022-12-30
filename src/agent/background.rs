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
	Png { filename: Filename }
}

impl Background {
	pub fn new(filename: &str) -> Background {
		let filename = Filename::new(filename, "png");
		match filename.extension.as_str() {
			"blk" => Background::Blk { filename },
			_ => Background::Png { filename }
		}
	}

	pub fn get_filename(&self) -> String {
		match self {
			Background::Blk { filename } => filename.to_string(),
			Background::Png { filename } => filename.to_string()
		}
	}

	pub fn get_title(&self) -> String {
		match self {
			Background::Blk { filename } => filename.title.clone(),
			Background::Png { filename } => filename.title.clone()
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
			Background::Png { filename } => {
				let filepath = format!("{}{}", path, filename);
				let image_data = ImageReader::open(&filepath)?;
				let image = image_data.decode()?;
				println!("  Got data from {}", &filepath);
				Ok(Bytes::from(blk::encode(image.into_rgba8())))
			}
		}
	}
}
