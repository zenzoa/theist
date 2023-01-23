use super::script::Script;
use super::sprite::Sprite;
use super::sound::Sound;
use super::catalogue::Catalogue;
use super::bodydata::BodyData;
use super::genetics::Genetics;

use std::error::Error;
use bytes::Bytes;

#[derive(Clone)]
pub enum CreaturesFile {
	Script(Script),
	Sprite(Sprite),
	Sound(Sound),
	Catalogue(Catalogue),
	BodyData(BodyData),
	Genetics(Genetics)
}

impl CreaturesFile {
	pub fn get_output_filename(&self) -> String {
		match self {
			CreaturesFile::Script(script) => script.get_output_filename(),
			CreaturesFile::Sprite(sprite) => sprite.get_output_filename(),
			CreaturesFile::Sound(sound) => sound.get_output_filename(),
			CreaturesFile::Catalogue(catalogue) => catalogue.get_output_filename(),
			CreaturesFile::BodyData(bodydata) => bodydata.get_output_filename(),
			CreaturesFile::Genetics(genetics) => genetics.get_output_filename()
		}
	}

	pub fn get_title(&self) -> String {
		match self {
			CreaturesFile::Script(script) => script.get_title(),
			CreaturesFile::Sprite(sprite) => sprite.get_title(),
			CreaturesFile::Sound(sound) => sound.get_title(),
			CreaturesFile::Catalogue(catalogue) => catalogue.get_title(),
			CreaturesFile::BodyData(bodydata) => bodydata.get_title(),
			CreaturesFile::Genetics(genetics) => genetics.get_title()
		}
	}

	pub fn get_extension(&self) -> String {
		match self {
			CreaturesFile::Script(script) => script.get_extension(),
			CreaturesFile::Sprite(sprite) => sprite.get_extension(),
			CreaturesFile::Sound(sound) => sound.get_extension(),
			CreaturesFile::Catalogue(catalogue) => catalogue.get_extension(),
			CreaturesFile::BodyData(bodydata) => bodydata.get_extension(),
			CreaturesFile::Genetics(genetics) => genetics.get_extension()
		}
	}

	pub fn get_data(&self) -> Option<Bytes> {
		match self {
			CreaturesFile::Script(script) => script.get_data(),
			CreaturesFile::Sprite(sprite) => sprite.get_data(),
			CreaturesFile::Sound(sound) => sound.get_data(),
			CreaturesFile::Catalogue(catalogue) => catalogue.get_data(),
			CreaturesFile::BodyData(bodydata) => bodydata.get_data(),
			CreaturesFile::Genetics(genetics) => genetics.get_data()
		}
	}

	pub fn fetch_data(&mut self, path: &String) -> Result<(), Box<dyn Error>> {
		match self {
			CreaturesFile::Script(script) => script.fetch_data(path),
			CreaturesFile::Sprite(sprite) => sprite.fetch_data(path),
			CreaturesFile::Sound(sound) => sound.fetch_data(path),
			CreaturesFile::Catalogue(catalogue) => catalogue.fetch_data(path),
			CreaturesFile::BodyData(bodydata) => bodydata.fetch_data(path),
			CreaturesFile::Genetics(genetics) => genetics.fetch_data(path)
		}
	}
}

// pub struct FileList {
// 	items: Vec<CreaturesFile>
// }

// impl FileList {
// 	pub fn get_item(&self, filename: String) -> Option<&CreaturesFile> {
// 		for item in &self.items {
// 			if item.get_output_filename() == filename {
// 				return Some(item);
// 			}
// 		}
// 		None
// 	}

// 	pub fn add_item(&mut self, new_item: CreaturesFile) {
// 		let mut file_already_in_list = false;
// 		for item in &self.items {
// 			if item.get_output_filename() == new_item.get_output_filename() {
// 				file_already_in_list = true;
// 			}
// 		}
// 		if !file_already_in_list {
// 			self.items.push(new_item);
// 		}
// 	}

// 	pub fn remove_item(&mut self, index: usize) {
// 		if index < self.items.len() {
// 			self.items.remove(index);
// 		}
// 	}

// 	pub fn move_item_up(&mut self, index: usize) {
// 		if index > 0 && index < self.items.len() {
// 			self.items.swap(index, index - 1);
// 		}
// 	}

// 	pub fn move_item_down(&mut self, index: usize) {
// 		if index + 1 < self.items.len() {
// 			self.items.swap(index, index + 1);
// 		}
// 	}
// }
