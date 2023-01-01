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

	pub fn set_name(&mut self, new_name: String) {
		if let Catalogue::Inline{ filename, .. } = self {
			filename.set_title(new_name);
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

	pub fn remove_entry(&mut self, index: usize) {
		if let Catalogue::Inline{ entries, .. } = self {
			if index < entries.len() {
				entries.remove(index);
			}
		}
	}

	pub fn move_entry_up(&mut self, index: usize) {
		if let Catalogue::Inline{ entries, .. } = self {
			if index > 0 && index < entries.len() {
				entries.swap(index, index - 1);
			}
		}
	}

	pub fn move_entry_down(&mut self, index: usize) {
		if let Catalogue::Inline{ entries, .. } = self {
			if index + 1 < entries.len() {
				entries.swap(index, index + 1);
			}
		}
	}

	pub fn set_entry_classifier(&mut self, index: usize, new_classifier: String) {
		if let Catalogue::Inline{ entries, .. } = self {
			if let Some(entry) = entries.get_mut(index) {
				entry.classifier = new_classifier;
			}
		}
	}

	pub fn set_entry_name(&mut self, index: usize, new_name: String) {
		if let Catalogue::Inline{ entries, .. } = self {
			if let Some(entry) = entries.get_mut(index) {
				entry.name = new_name;
			}
		}
	}

	pub fn set_entry_description(&mut self, index: usize, new_description: String) {
		if let Catalogue::Inline{ entries, .. } = self {
			if let Some(entry) = entries.get_mut(index) {
				entry.description = new_description;
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

#[derive(Clone)]
pub struct CatalogueList(Vec<Catalogue>);

impl CatalogueList {
	pub fn new() -> CatalogueList {
		CatalogueList(Vec::new())
	}

	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	pub fn len(&self) -> usize {
		self.0.len()
	}

	pub fn iter(&self) -> std::slice::Iter<'_, Catalogue> {
		self.0.iter()
	}

	pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Catalogue> {
		self.0.iter_mut()
	}

	pub fn get(&self, index: usize) -> Option<&Catalogue> {
		self.0.get(index)
	}

	pub fn get_mut(&mut self, index: usize) -> Option<&mut Catalogue> {
		self.0.get_mut(index)
	}

	pub fn push(&mut self, catalogue: Catalogue) {
		self.0.push(catalogue)
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
