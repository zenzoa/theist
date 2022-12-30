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
