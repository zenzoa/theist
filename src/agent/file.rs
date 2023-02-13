use super::script::Script;
use super::sprite::Sprite;
use super::sound::Sound;
use super::catalogue::Catalogue;
use super::bodydata::BodyData;
use super::genetics::Genetics;
use crate::agent::tag::Tag;
use crate::file_helper;

use std::error::Error;
use bytes::Bytes;

#[derive(Debug, Clone)]
pub enum FileType {
	Script,
	Sprite,
	Sound,
	Catalogue,
	BodyData,
	Genetics
}

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
	pub fn get_input_filename(&self) -> String {
		match self {
			CreaturesFile::Script(script) => script.input_filename.to_string(),
			CreaturesFile::Sprite(sprite) => match sprite {
					Sprite::Raw{ input_filename, .. } => input_filename.to_string(),
					Sprite::Png{ input_filename, frames, .. } =>
						match frames.len() {
							1 => input_filename.to_string(),
							_ => "".to_string()
						}
				},
			CreaturesFile::Sound(sound) => sound.input_filename.to_string(),
			CreaturesFile::Catalogue(catalogue) => match catalogue {
					Catalogue::Raw{ input_filename, .. } => input_filename.to_string(),
					Catalogue::Inline{ .. } => "".to_string()
				},
			CreaturesFile::BodyData(bodydata) => bodydata.input_filename.to_string(),
			CreaturesFile::Genetics(genetics) => genetics.input_filename.to_string()
		}
	}

	pub fn get_output_filename(&self) -> String {
		match self {
			CreaturesFile::Script(script) => script.output_filename.to_string(),
			CreaturesFile::Sprite(sprite) => match sprite {
					Sprite::Raw{ output_filename, .. } => output_filename.to_string(),
					Sprite::Png{ output_filename, .. } => output_filename.to_string()
				},
			CreaturesFile::Sound(sound) => sound.output_filename.to_string(),
			CreaturesFile::Catalogue(catalogue) => match catalogue {
					Catalogue::Raw{ output_filename, .. } => output_filename.to_string(),
					Catalogue::Inline{ output_filename, .. } => output_filename.to_string()
				},
			CreaturesFile::BodyData(bodydata) => bodydata.output_filename.to_string(),
			CreaturesFile::Genetics(genetics) => genetics.output_filename.to_string()
		}
	}

	pub fn get_output_filename_ref(&self) -> &String {
		match self {
			CreaturesFile::Script(script) => &script.output_filename,
			CreaturesFile::Sprite(sprite) => match sprite {
					Sprite::Raw{ output_filename, .. } => &output_filename,
					Sprite::Png{ output_filename, .. } => &output_filename
				},
			CreaturesFile::Sound(sound) => &sound.output_filename,
			CreaturesFile::Catalogue(catalogue) => match catalogue {
					Catalogue::Raw{ output_filename, .. } => &output_filename,
					Catalogue::Inline{ output_filename, .. } => &output_filename
				},
			CreaturesFile::BodyData(bodydata) => &bodydata.output_filename,
			CreaturesFile::Genetics(genetics) => &genetics.output_filename
		}
	}

	pub fn get_title(&self) -> String {
		file_helper::title(&self.get_output_filename())
	}

	pub fn get_extension(&self) -> String {
		file_helper::extension(&self.get_output_filename())
	}

	pub fn get_category_id(&self) -> usize {
		match self.get_extension().as_str() {
			"cos" => 0,
			"c16" => 1,
			"s16" => 2,
			"blk" => 3,
			"wav" => 4,
			"catalogue" => 5,
			"att" => 6,
			"gen" => 7,
			"gno" => 8,
			_ => 9
		}
	}

	pub fn get_filetype(&self) -> FileType {
		match self {
			CreaturesFile::Script(_script) => FileType::Script,
			CreaturesFile::Sprite(_sprite) => FileType::Sprite,
			CreaturesFile::Sound(_sound) => FileType::Sound,
			CreaturesFile::Catalogue(_catalogue) => FileType::Catalogue,
			CreaturesFile::BodyData(_bodydata) => FileType::BodyData,
			CreaturesFile::Genetics(_genetics) => FileType::Genetics
		}
	}

	pub fn get_data(&self) -> Option<Bytes> {
		match self {
			CreaturesFile::Script(script) => script.data.clone(),
			CreaturesFile::Sprite(sprite) => match sprite {
					Sprite::Raw{ data, .. } => data.clone(),
					Sprite::Png{ data, .. } => data.clone()
				},
			CreaturesFile::Sound(sound) => sound.data.clone(),
			CreaturesFile::Catalogue(catalogue) => match catalogue {
					Catalogue::Raw{ data, .. } => data.clone(),
					Catalogue::Inline{ data, .. } => data.clone()
				},
			CreaturesFile::BodyData(bodydata) => bodydata.data.clone(),
			CreaturesFile::Genetics(genetics) => genetics.data.clone()
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

pub fn lookup_file_index(files: &[CreaturesFile], filename: &String) -> Option<usize> {
	for (i, file) in files.iter().enumerate() {
		if &file.get_output_filename() == filename {
			return Some(i);
		}
	}
	None
}

pub fn only_used_files(tags: &Vec<Tag>, files: &Vec<CreaturesFile>) -> Vec<CreaturesFile> {
	let mut used_files: Vec<CreaturesFile> = Vec::new();
	for (i, file) in files.iter().enumerate() {
		let mut is_used = false;
		for tag in tags {
			if tag.has_file(&file.get_filetype(), i) {
				is_used = true;
				break;
			}
		}
		if is_used {
			used_files.push(file.clone());
		}
	}
	used_files
}
