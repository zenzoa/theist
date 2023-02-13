use super::agent_tag::{ AgentTag, SupportedGame };
use super::egg_tag::EggTag;
use super::free_tag::FreeTag;
use crate::agent::file::{ CreaturesFile, FileType };
use crate::source::agent_tag::encode as encode_agent_tag;
use crate::source::egg_tag::encode as encode_egg_tag;
use crate::source::free_tag::encode as encode_free_tag;
use crate::pray::agent_block::write_agent_block;
use crate::pray::egg_block::write_egg_block;
use crate::pray::free_block::write_free_block;

use std::error::Error;
use bytes::Bytes;

#[derive(Clone)]
pub enum Tag {
	Agent(AgentTag),
	Egg(EggTag),
	Free(FreeTag)
}

impl Tag {
	pub fn get_type(&self) -> String {
		match self {
			Tag::Agent(_tag) => "agent".to_string(),
			Tag::Egg(_tag) => "egg".to_string(),
			Tag::Free(tag) => tag.block_type.to_string()
		}
	}

	pub fn get_name(&self) -> &String {
		match self {
			Tag::Agent(tag) => &tag.name,
			Tag::Egg(tag) => &tag.name,
			Tag::Free(tag) => &tag.name
		}
	}

	pub fn does_use_all_files(&self) -> bool {
		match self {
			Tag::Agent(tag) => tag.use_all_files,
			Tag::Egg(tag) => tag.use_all_files,
			_ => false
		}
	}

	pub fn get_file_list(&self, filetype: &FileType) -> Option<&Vec<usize>> {
		match self {
			Tag::Agent(tag) => {
				match filetype {
					FileType::Script => Some(&tag.scripts),
					FileType::Sprite => Some(&tag.sprites),
					FileType::Sound => Some(&tag.sounds),
					FileType::Catalogue => Some(&tag.catalogues),
					_ => None
				}
			},
			Tag::Egg(tag) => {
				match filetype {
					FileType::Sprite => Some(&tag.sprites),
					FileType::BodyData => Some(&tag.bodydata),
					FileType::Genetics => Some(&tag.genetics),
					_ => None
				}
			},
			_ => None
		}
	}

	pub fn get_file_list_mut(&mut self, filetype: &FileType) -> Option<&mut Vec<usize>> {
		match self {
			Tag::Agent(tag) => {
				match filetype {
					FileType::Script => Some(&mut tag.scripts),
					FileType::Sprite => Some(&mut tag.sprites),
					FileType::Sound => Some(&mut tag.sounds),
					FileType::Catalogue => Some(&mut tag.catalogues),
					_ => None
				}
			},
			Tag::Egg(tag) => {
				match filetype {
					FileType::Sprite => Some(&mut tag.sprites),
					FileType::BodyData => Some(&mut tag.bodydata),
					FileType::Genetics => Some(&mut tag.genetics),
					_ => None
				}
			},
			_ => None
		}
	}

	pub fn has_file(&self, filetype: &FileType, file_index: usize) -> bool {
		if let Some(file_list) = self.get_file_list(filetype) {
			for file in file_list {
				if file == &file_index {
					return true;
				}
			}
		}
		false
	}

	pub fn add_file(&mut self, filetype: &FileType, file_index: usize) -> Option<usize> {
		if let Some(file_list) = self.get_file_list_mut(filetype) {
			file_list.push(file_index);
			return Some(file_list.len() - 1);
		}
		None
	}

	pub fn add_files(&mut self, files: &[CreaturesFile]) {
		for (i, file) in files.iter().enumerate() {
			self.add_file(&file.get_filetype(), i);
		}
	}

	pub fn remove_file(&mut self, filetype: &FileType, ref_index: usize) {
		if let Some(file_list) = self.get_file_list_mut(filetype) {
			file_list.remove(ref_index);
		}
	}

	pub fn move_file_up(&mut self, filetype: &FileType, ref_index: usize) -> usize {
		if let Some(file_list) = self.get_file_list_mut(filetype) {
			if ref_index > 0 && ref_index < file_list.len() {
				file_list.swap(ref_index, ref_index - 1);
				return ref_index - 1;
			}
		}
		ref_index
	}

	pub fn move_file_down(&mut self, filetype: &FileType, ref_index: usize) -> usize {
		if let Some(file_list) = self.get_file_list_mut(filetype) {
			if ref_index + 1 < file_list.len() {
				file_list.swap(ref_index, ref_index + 1);
				return ref_index + 1;
			}
		}
		ref_index
	}

	pub fn write_block(&self, files: &[CreaturesFile]) -> Result<Bytes, Box<dyn Error>> {
		match self {
			Tag::Agent(tag) => write_agent_block(tag, files),
			Tag::Egg(tag) => write_egg_block(tag, files),
			Tag::Free(tag) => write_free_block(tag, files)
		}
	}

	pub fn encode(&self, files: &[CreaturesFile]) -> String {
		match self {
			Tag::Agent(tag) => encode_agent_tag(tag, files),
			Tag::Egg(tag) => encode_egg_tag(tag, files),
			Tag::Free(tag) => encode_free_tag(tag)
		}
	}
}

pub fn split_tags(base_tags: &Vec<Tag>, base_files: &Vec<CreaturesFile>) -> (Vec<Tag>, Vec<CreaturesFile>) {
	let mut tags: Vec<Tag> = Vec::new();
	let mut files: Vec<CreaturesFile> = base_files.clone();

	for tag in base_tags {
		if let Tag::Agent(agent_tag) = tag {
			if agent_tag.supported_game == SupportedGame::C3DS {
				let mut c3_tag = agent_tag.clone();
				c3_tag.name.push_str(" C3");

				let mut ds_tag = agent_tag.clone();
				ds_tag.name.push_str(" DS");

				for (j, script) in agent_tag.scripts.iter().enumerate() {
					if let Some(CreaturesFile::Script(script_file)) = base_files.get(script.clone()) {
						c3_tag.scripts.remove(j);
						ds_tag.scripts.remove(j);

						let mut c3_file = script_file.clone();
						c3_file.output_filename = format!("{} C3.cos", &script_file.get_title());
						files.push(CreaturesFile::Script(c3_file));
						c3_tag.scripts.push(files.len() - 1);

						let mut ds_file = script_file.clone();
						ds_file.output_filename = format!("{} DS.cos", &script_file.get_title());
						files.push(CreaturesFile::Script(ds_file));
						ds_tag.scripts.push(files.len() - 1);
					}
				}

				tags.push(Tag::Agent(c3_tag));
				tags.push(Tag::Agent(ds_tag));

			} else {
				tags.push(tag.clone());
			}

		} else {
			tags.push(tag.clone());
		}
	}

	(tags, files)
}
