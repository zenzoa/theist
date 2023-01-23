use super::file;
use crate::agent::file::CreaturesFile;
use crate::agent::tag::Tag;

use std::error::Error;

pub fn encode(tags: Vec<Box<dyn Tag>>, files: Vec<CreaturesFile>) -> Result<String, Box<dyn Error>> {
	let mut content = String::new();

	content.push_str("files\n");
	for file in &files {
		content.push_str(&file::encode(file)?);
	}
	content.push('\n');

	for tag in &tags {
		content.push_str(&tag.encode());
	}

	Ok(content)
}

