pub mod tag;
pub mod agent_tag;
pub mod egg_tag;
pub mod script;
pub mod sprite;
pub mod background;
pub mod sound;
pub mod catalogue;
pub mod genetics;
pub mod body_data;
pub mod encode;
pub mod decode;

use tag::*;
use agent_tag::*;
use egg_tag::*;
use script::*;
use sprite::*;
use background::*;
use sound::*;
use catalogue::*;
use genetics::*;
use body_data::*;
use encode::*;

use crate::pray;

use std::fmt;
use std::str;
use std::error::Error;
use regex::Regex;
use bytes::Bytes;

pub struct FileData {
	pub name: String,
	pub data: Bytes
}

#[derive(Clone)]
pub struct Filename {
	pub string: String,
	pub title: String,
	pub extension: String
}

impl Filename {
	pub fn new(filename_string: &str) -> Filename {
		let filename_pattern = Regex::new(r"^(.+)\.(.+)$").unwrap();
		match filename_pattern.captures(filename_string) {
			None => Filename {
				string: String::from(""),
				title: String::from(""),
				extension: String::from("")
			},
			Some(captures) => Filename {
				string: String::from(filename_string),
				title: String::from(&captures[1]),
				extension: String::from(&captures[2])
			}
		}
	}

	pub fn set_title(&mut self, new_title: String) {
		self.title = new_title;
		self.string = format!("{}.{}", &self.title, &self.extension);
	}

	pub fn with_extension(&self, new_extension: &str) -> Filename {
		Filename {
			string: format!("{}.{}", &self.title, &new_extension),
			title: self.title.clone(),
			extension: new_extension.to_string()
		}
	}
}

impl fmt::Display for Filename {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", &self.string)
	}
}

#[derive(Clone)]
pub enum SupportedGame {
	C3,
	DS,
	C3DS
}

impl fmt::Display for SupportedGame {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match &self {
			SupportedGame::C3 => write!(f, "c3"),
			SupportedGame::DS => write!(f, "ds"),
			SupportedGame::C3DS => write!(f, "c3ds"),
		}
	}
}

impl SupportedGame {
	pub fn new(s: &str) -> SupportedGame {
		match s {
			"c3" => SupportedGame::C3,
			"C3" => SupportedGame::C3,
			"ds" => SupportedGame::DS,
			"DS" => SupportedGame::DS,
			_ => SupportedGame::C3DS
		}
	}
}

pub fn compile(mut tags: Vec<Tag>) -> Bytes {
	for tag in &mut tags {
		tag.add_data();
	}
	println!();
	let tags = split_tags(&tags);
	println!();
	pray::encode(&tags)
}

pub fn decompile(contents: &[u8], filename: &str) -> Result<Vec<FileData>, Box<dyn Error>> {
	let (tags, mut files) = pray::decode(contents)?;
	files.push(FileData {
		name: format!("{}.the", filename),
		data: encode_source(tags)
	});
	Ok(files)
}
