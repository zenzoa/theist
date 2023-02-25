use super::{ agent_tag, egg_tag, free_tag, file };
use crate::agent::file::CreaturesFile;
use crate::agent::tag::Tag;
use crate::agent::agent_tag::SupportedGame;

use std::str;
use std::error::Error;

pub struct DecodeResult {
	pub tags: Vec<Tag>,
	pub files: Vec<CreaturesFile>
}

pub fn decode(contents: &str) -> Result<DecodeResult, Box<dyn Error>> {
	let mut tags: Vec<Tag> = Vec::new();
	let mut files: Vec<CreaturesFile> = Vec::new();

	let lines: Vec<&str> = contents.lines().collect();

	// first get list of files
	let mut i = 0;
	while i < lines.len() {
		let tokens = parse_tokens(lines[i]);
		for token in &tokens {
			if token.as_str() == "files" {
				let (mut new_files, lines_to_skip) = file::decode(Vec::from(&lines[i+1..]));
				files.append(&mut new_files);
				i += lines_to_skip;
			}
		}
		i += 1;
	}

	// then decode tags
	i = 0;
	while i < lines.len() {
		let tokens = parse_tokens(lines[i]);
		for token in &tokens {
			match token.as_str() {
				"agent" => {
					if let Some(value) = tokens.get(1) {
						let name = value.to_string();
						let mut supported_game = SupportedGame::C3DS;
						if let Some(value2) = tokens.get(2) {
							supported_game = match value2.as_str() {
								"c3" => SupportedGame::C3,
								"ds" => SupportedGame::DS,
								_ => SupportedGame::C3DS
							};
							if let Some(value3) = tokens.get(3) {
								if (value2 == "c3" && value3 == "ds") || (value2 == "ds" && value3 == "c3") {
									supported_game = SupportedGame::C3DS;
								}
							}
						}
						let (agent_tag, lines_to_skip) = agent_tag::decode(Vec::from(&lines[i+1..]), name, supported_game, &files);
						tags.push(Tag::Agent(agent_tag));
						i += lines_to_skip;
					}
				},

				"egg" => {
					if let Some(value) = tokens.get(1) {
						let name = value.to_string();
						let (egg_tag, lines_to_skip) = egg_tag::decode(Vec::from(&lines[i+1..]), name, &files);
						tags.push(Tag::Egg(egg_tag));
						i += lines_to_skip;
					}
				},

				"other" => {
					if let Some(value) = tokens.get(1) {
						let name = value.to_string();
						let block_type = if let Some(value2) = tokens.get(2) {
							value2.to_string()
						} else {
							String::new()
						};
						let (free_tag, lines_to_skip) = free_tag::decode(Vec::from(&lines[i+1..]), name, block_type);
						tags.push(Tag::Free(free_tag));
						i += lines_to_skip;
					}
				},

				_ => ()
			}
		}

		i += 1;
	}

	for tag in &mut tags {
		if tag.does_use_all_files() {
			tag.add_files(&files);
		}
	}

	Ok(DecodeResult{ tags, files })
}

pub fn parse_tokens(s: &str) -> Vec<String> {
	let mut tokens: Vec<String> = Vec::new();
	let mut current_token = String::from("");
	let mut is_in_quote = false;
	let mut is_escaped = false;
	for c in s.chars() {
		if is_escaped {
			current_token.push(c);
			is_escaped = false;
		} else {
			match c {
				' ' => {
					if is_in_quote {
						current_token.push(c);
					} else if !current_token.is_empty() {
						tokens.push(current_token.clone());
						current_token.clear();
					}
				},
				'"' => {
					if is_in_quote {
						is_in_quote = false;
					} else {
						is_in_quote = true;
					}
				},
				'\\' => {
					is_escaped = true;
				},
				'\t' => (),
				'\r' => (),
				_ => {
					current_token.push(c);
				}
			}
		}
	}
	if !current_token.is_empty() {
		tokens.push(current_token.clone());
	}
	tokens
}
