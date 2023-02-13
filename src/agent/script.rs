use super::file::FileType;
use crate::error::create_error;
use crate::file_helper;

use std::fs;
use std::error::Error;
use bytes::Bytes;

#[derive(Clone)]
pub struct Script {
	pub filetype: FileType,
	pub output_filename: String,
	pub input_filename: String,
	pub data: Option<Bytes>
}

impl Script {
	pub fn new_from_data(input_filename: &String, data: &mut Bytes) -> Result<Script, Box<dyn Error>> {
		if file_helper::extension(input_filename) == "cos" {
			Ok(Script{
				filetype: FileType::Script,
				output_filename: file_helper::filename(input_filename),
				input_filename: input_filename.to_string(),
				data: Some(data.clone())
			})
		} else {
			Err(create_error("Unrecognized file type. Script must be a COS file."))
		}
	}

	pub fn get_output_filename(&self) -> String {
		self.output_filename.to_string()
	}

	pub fn get_title(&self) -> String {
		file_helper::title(&self.output_filename)
	}

	pub fn get_data(&self) -> Option<Bytes> {
		self.data.clone()
	}

	pub fn fetch_data(&mut self, path: &String) -> Result<(), Box<dyn Error>> {
		let contents = fs::read(format!("{}{}", path, self.input_filename))?;
		self.data = Some(Bytes::copy_from_slice(&contents));
		Ok(())
	}
}

