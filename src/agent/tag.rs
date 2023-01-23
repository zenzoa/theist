use std::error::Error;
use bytes::Bytes;
use crate::agent::file::CreaturesFile;

pub trait Tag {
	fn get_type(&self) -> String;
	fn get_name(&self) -> String;
	fn get_scripts(&self) -> Vec<String> { Vec::new() }
	fn does_use_all_files(&self) -> bool { false }
	fn add_files(&mut self, _files: &[CreaturesFile]) {}
	fn write_block(&self, files: &[CreaturesFile]) -> Result<Bytes, Box<dyn Error>>;
	fn encode(&self) -> String;
	fn split(&self) -> Vec<Box<dyn Tag>>;
}

pub fn split_tags(base_tags: Vec<Box<dyn Tag>>, base_files: Vec<CreaturesFile>) -> (Vec<Box<dyn Tag>>, Vec<CreaturesFile>) {
	let mut tags: Vec<Box<dyn Tag>> = Vec::new();
	let mut files: Vec<CreaturesFile> = Vec::new();

	for tag in &base_tags {
		let scripts_before = tag.get_scripts();

		let mut new_tags = tag.split();

		if new_tags.len() > 1 {
			for file in &base_files {
				if let CreaturesFile::Script(script) = file {
					let mut script_in_list = false;

					for before_script in &scripts_before {
						if before_script == &script.get_output_filename() {
							script_in_list = true;
						}
					}

					if script_in_list {
						let mut c3_file = script.clone();
						c3_file.output_filename = format!("{} C3.cos", &c3_file.get_title());
						files.push(CreaturesFile::Script(c3_file));

						let mut ds_file = script.clone();
						ds_file.output_filename = format!("{} DS.cos", &ds_file.get_title());
						files.push(CreaturesFile::Script(ds_file));

					} else {
						files.push(file.clone());
					}

				} else {
					files.push(file.clone());
				}
			}
		}

		tags.append(&mut new_tags);
	}

	(tags, files)
}
