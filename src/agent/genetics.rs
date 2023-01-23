use crate::error::create_error;
use crate::file_helper;

use std::fs;
use std::error::Error;
use bytes::Bytes;

#[derive(Clone)]
pub struct Genetics {
	pub output_filename: String,
	pub input_filename: String,
	pub data: Option<Bytes>
}

impl Genetics {
	// pub fn new(input_filename: &String) -> Result<Genetics, Box<dyn Error>> {
	// 	match file_helper::extension(input_filename).as_str() {
	// 		"gen" => {
	// 			Ok(Genetics{
	// 				output_filename: file_helper::filename(input_filename),
	// 				input_filename: input_filename.to_string(),
	// 				data: None
	// 			})
	// 		},
	// 		"gno" => {
	// 			Ok(Genetics{
	// 				output_filename: file_helper::filename(input_filename),
	// 				input_filename: input_filename.to_string(),
	// 				data: None
	// 			})
	// 		},
	// 		_ => {
	// 			Err(create_error("Unrecognized file type. Genetics file must be a GEN or GNO."))
	// 		}
	// 	}
	// }

	pub fn new_from_data(input_filename: &String, data: &mut Bytes) -> Result<Genetics, Box<dyn Error>> {
		match file_helper::extension(input_filename).as_str() {
			"gen" => {
				Ok(Genetics{
					output_filename: file_helper::filename(input_filename),
					input_filename: input_filename.to_string(),
					data: Some(data.clone())
				})
			},
			"gno" => {
				Ok(Genetics{
					output_filename: file_helper::filename(input_filename),
					input_filename: input_filename.to_string(),
					data: Some(data.clone())
				})
			},
			_ => {
				Err(create_error("Unrecognized file type. Genetics file must be a GEN or GNO."))
			}
		}
	}

	pub fn get_output_filename(&self) -> String {
		self.output_filename.to_string()
	}

	pub fn get_title(&self) -> String {
		file_helper::title(&self.output_filename)
	}

	pub fn get_extension(&self) -> String {
		file_helper::extension(&self.get_output_filename())
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

