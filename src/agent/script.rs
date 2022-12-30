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
}
