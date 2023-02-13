use super::file::FileType;
use crate::error::create_error;
use crate::file_helper;

use std::fs;
use std::error::Error;
use bytes::Bytes;

#[derive(Clone)]
pub enum Catalogue {
	Raw {
		filetype: FileType,
		output_filename: String,
		input_filename: String,
		data: Option<Bytes>
	},
	Inline {
		filetype: FileType,
		output_filename: String,
		entries: Vec<CatalogueEntry>,
		data: Option<Bytes>
	}
}

#[derive(Clone)]
pub struct CatalogueEntry {
	pub name: String,
	pub classifier: String,
	pub description: String
}

impl Catalogue {
	pub fn new(input_filename: &String) -> Result<Catalogue, Box<dyn Error>> {
		if file_helper::extension(input_filename) == "catalogue" {
			Ok(Catalogue::Raw{
				filetype: FileType::Catalogue,
				output_filename: file_helper::filename(input_filename),
				input_filename: input_filename.to_string(),
				data: None
			})
		} else {
			Ok(Catalogue::Inline{
				filetype: FileType::Catalogue,
				output_filename: format!("{}.catalogue", file_helper::title(input_filename)),
				entries: Vec::new(),
				data: None
			})
		}
	}

	pub fn new_from_data(input_filename: &String, data: &mut Bytes) -> Result<Catalogue, Box<dyn Error>> {
		if file_helper::extension(input_filename) == "catalogue" {
			Ok(Catalogue::Raw{
				filetype: FileType::Catalogue,
				output_filename: file_helper::filename(input_filename),
				input_filename: input_filename.to_string(),
				data: Some(data.clone())
			})
		} else {
			Err(create_error("Unrecognized file type. Catalogue must be a CATALOGUE file."))
		}
	}

	pub fn add_entry(&mut self, new_entry: CatalogueEntry) {
		if let Catalogue::Inline{ entries, .. } = self {
			entries.push(new_entry);
		}
	}

	pub fn remove_entry(&mut self, index: usize) -> bool {
		if let Catalogue::Inline{ entries, .. } = self {
			if index < entries.len() {
				entries.remove(index);
				return true;
			}
		}
		false
	}

	pub fn move_entry_up(&mut self, index: usize) -> bool {
		if let Catalogue::Inline{ entries, .. } = self {
			if index > 0 && index < entries.len() {
				entries.swap(index, index - 1);
				return true;
			}
		}
		false
	}

	pub fn move_entry_down(&mut self, index: usize) -> bool {
		if let Catalogue::Inline{ entries, .. } = self {
			if index + 1 < entries.len() {
				entries.swap(index, index + 1);
				return true;
			}
		}
		false
	}

	pub fn get_output_filename(&self) -> String {
		match self {
			Catalogue::Raw{ output_filename, .. } => output_filename.to_string(),
			Catalogue::Inline{ output_filename, .. } => output_filename.to_string()
		}
	}

	pub fn fetch_data(&mut self, path: &String) -> Result<(), Box<dyn Error>> {
		match self {
			Catalogue::Raw{ input_filename, data, .. } => {
				let contents = fs::read(format!("{}{}", path, input_filename))?;
				*data = Some(Bytes::copy_from_slice(&contents));
				Ok(())
			},
			Catalogue::Inline{ entries, data, .. } => {
				let mut contents = String::new();
				for entry in entries {
					contents.push_str(&format!(
						"TAG \"Agent Help {}\"\n\"{}\"\n\"{}\"\n\n",
						entry.classifier,
						entry.name,
						entry.description.replace('"', "\\\"")
					));
				}
				*data = Some(Bytes::copy_from_slice(contents.as_bytes()));
				Ok(())
			}
		}
	}
}
