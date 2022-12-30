use crate::agent::*;

use std::fs;
use std::str;
use std::error::Error;
use bytes::Bytes;

#[derive(Clone)]
pub enum Catalogue {
	File { filename: Filename },
	Inline { filename: Filename, entries: Vec<CatalogueEntry> }
}

impl Catalogue {
	pub fn new(filename: &str) -> Catalogue {
		Catalogue::File {
			filename: Filename::new(filename)
		}
	}

	pub fn add_entry(&mut self, entry: CatalogueEntry) {
		match self {
			Catalogue::File { filename } => {
				*self = Catalogue::Inline {
					filename: filename.clone(),
					entries: vec![ entry ]
				}
			},
			Catalogue::Inline { entries, .. } => {
				entries.push(entry);
			}
		}
	}

	pub fn get_filename(&self) -> String {
		match self {
			Catalogue::File { filename } => filename.to_string(),
			Catalogue::Inline { filename, .. } => filename.to_string()
		}
	}

	pub fn get_data(&self, path: &str) -> Result<Bytes, Box<dyn Error>> {
		match self {
			Catalogue::File { filename } => {
				let filepath = format!("{}{}", path, filename);
				let contents = fs::read(&filepath)?;
				println!("  Got data from {}", &filepath);
				Ok(Bytes::copy_from_slice(&contents))
			},
			Catalogue::Inline { filename, entries } => {
				let mut contents = String::new();
				for entry in entries {
					contents += format!(
						"TAG \"Agent Help {}\"\n\"{}\"\n\"{}\"\n\n",
						entry.classifier,
						entry.name,
						entry.description
					).as_str();
				}
				println!("  Catalogue created: {}", filename);
				Ok(Bytes::copy_from_slice(contents.as_bytes()))
			}
		}
	}
}

#[derive(Clone)]
pub struct CatalogueEntry {
	pub classifier: String,
	pub name: String,
	pub description: String
}

impl CatalogueEntry {
	pub fn new(classifier: &str, name: &str, description: &str) -> CatalogueEntry {
		CatalogueEntry {
			classifier: String::from(classifier),
			name: String::from(name),
			description: String::from(description)
		}
	}
}
