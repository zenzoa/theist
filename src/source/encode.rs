use super::file;
use crate::agent::file::{ CreaturesFile, only_used_files };
use crate::agent::tag::Tag;

use std::error::Error;

pub fn encode(tags: &Vec<Tag>, files: &mut [CreaturesFile]) -> Result<String, Box<dyn Error>> {
	let mut content = String::new();

	for tag in tags {
		content.push_str(&tag.encode(files));
	}

	content.push('\n');

	let mut files = only_used_files(tags, files);
	files.sort_by_key(|f| format!("{} {}", f.get_category_id(), f.get_output_filename()));

	content.push_str("files\n");
	for file in &mut files {
		content.push_str(&file::encode(file)?);
	}

	Ok(content)
}
