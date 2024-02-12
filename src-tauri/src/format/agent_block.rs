use std::{ str, error::Error };
use bytes::{ Bytes, BytesMut };

use super::file_block::File;
use super::pray::{
	Tag,
	Block,
	BlockHeader,
	read_tag_block,
	write_block_header,
	write_tag_block,
	get_dependencies,
	write_dependencies,
	file_stem
};

#[derive(Clone, serde::Serialize)]
pub struct Agent {
	pub name: String,
	pub game_support: GameSupport,
	pub descriptions: Vec<Description>,
	pub bioenergy: u32,
	pub web_label: String,
	pub web_url: String,
	pub animation_file: String,
	pub animation_string: String,
	pub sprite_first_image: u32,
	pub remove_script: String,
	pub dependencies: Vec<String>,
}

#[derive(Clone, serde::Serialize)]
pub enum Language {
	English,
	German, //de
	Spanish, //es
	French, //fr
	Italian, //it
	Dutch, //nl
}

#[derive(Clone, serde::Serialize)]
pub struct Description {
	pub language: Language,
	pub text: String
}

impl Description {
	pub fn new(language: Language, text: String) -> Self {
		Self { language, text }
	}
}

#[derive(Clone, PartialEq, serde::Serialize)]
pub enum GameSupport {
	Creatures3,
	DockingStation
}

pub fn read_agent_block(buffer: &mut Bytes, name: &str, game_support: GameSupport) -> Result<Vec<Block>, Box<dyn Error>> {
	let tag = read_tag_block(buffer)?;
	let mut agent_blocks: Vec<Block> = Vec::new();

	let mut dependencies = get_dependencies(&tag);

	for script_block in get_scripts(&tag, name) {
		if let Block::File(script) = &script_block {
			dependencies.push(format!("{}.cos", script.name));
		}
		agent_blocks.push(script_block);
	}

	agent_blocks.push(Block::Agent(Agent {
		game_support,
		name: name.to_string(),
		descriptions: get_descriptions(&tag),
		bioenergy: *tag.int_values.get("Agent Bioenergy Value").unwrap_or(&0) as u32,
		web_label: tag.str_values.get("Web Label").unwrap_or(&String::new()).clone(),
		web_url: tag.str_values.get("Web URL").unwrap_or(&String::new()).clone(),
		animation_file: tag.str_values.get("Agent Animation File").unwrap_or(&String::new()).clone(),
		animation_string: tag.str_values.get("Agent Animation String").unwrap_or(&String::new()).clone(),
		sprite_first_image: *tag.int_values.get("Agent Sprite First Image").unwrap_or(&0) as u32,
		remove_script: tag.str_values.get("Remove script").unwrap_or(&String::new()).clone(),
		dependencies
	}));

	Ok(agent_blocks)
}

pub fn write_agent_block(agent_block: &Agent, dependencies: &Vec<File>) -> Result<Bytes, Box<dyn Error>> {
	let mut tag_scripts: Vec<&File> = Vec::new();
	let mut tag_dependencies: Vec<&File> = Vec::new();
	for dependency in dependencies {
		if agent_block.dependencies.contains(&dependency.filename()) {
			if dependency.extension == "cos" {
				tag_scripts.push(dependency);
			} else {
				tag_dependencies.push(dependency);
			}
		}
	}

	let mut int_values: Vec<(String, u32)> = Vec::new();
	let mut str_values: Vec<(String, String)> = Vec::new();

	int_values.push(("Agent Type".to_string(), 0));

	str_values.push(("Agent Animation Gallery".to_string(), file_stem(&agent_block.animation_file)));
	str_values.push(("Agent Animation File".to_string(), agent_block.animation_file.clone()));
	str_values.push(("Agent Animation String".to_string(), agent_block.animation_string.clone()));

	if agent_block.game_support == GameSupport::Creatures3 {
		int_values.push(("Agent Bioenergy Value".to_string(), agent_block.bioenergy));
	} else {
		int_values.push(("Agent Sprite First Image".to_string(), agent_block.sprite_first_image));
		str_values.push(("Web Label".to_string(), agent_block.web_label.clone()));
		str_values.push(("Web URL".to_string(), agent_block.web_url.clone()));
		for description in &agent_block.descriptions {
			let key = match description.language {
				Language::English => "Agent Description",
				Language::German => "Agent Description-de",
				Language::Spanish => "Agent Description-es",
				Language::French => "Agent Description-fr",
				Language::Italian => "Agent Description-it",
				Language::Dutch => "Agent Description-nl",
			};
			str_values.push((key.to_string(), description.text.clone()));
		}
	}

	write_dependencies(&mut int_values, &mut str_values, &tag_dependencies);

	str_values.push(("Remove script".to_string(), agent_block.remove_script.clone()));

	int_values.push(("Script Count".to_string(), tag_scripts.len() as u32));
	for (i, script) in tag_scripts.iter().enumerate() {
		str_values.push((format!("Script {}", i + 1), str::from_utf8(&script.data)?.to_string()));
	}

	let content_buffer = write_tag_block(&int_values, &str_values);

	let block_header = BlockHeader {
		id: (if agent_block.game_support == GameSupport::Creatures3{ "AGNT" } else { "DSAG" }).to_string(),
		name: agent_block.name.clone(),
		size_compressed: content_buffer.len(),
		size_uncompressed: content_buffer.len(),
		is_compressed: false
	};

	let mut buffer = BytesMut::new();
	buffer.extend_from_slice(&write_block_header(block_header));
	buffer.extend_from_slice(&content_buffer);
	Ok(buffer.freeze())
}

pub fn get_scripts(tag: &Tag, block_name: &str) -> Vec<Block> {
	let mut scripts: Vec<Block> = Vec::new();
	let num_scripts = *tag.int_values.get("Script Count").unwrap_or(&0) as usize;
	for i in 1..(num_scripts + 1) {
		let script_text = tag.str_values.get(&format!("Script {}", i));
		if let Some(script_text) = script_text {
			scripts.push(Block::File(File {
				name: if i == 1 { block_name.to_string() } else { format!("{} {}", block_name, i) },
				extension: "cos".to_string(),
				data: Bytes::from(script_text.clone()),
				is_checked: false
			}));
		}
	}
	scripts
}

pub fn get_descriptions(tag: &Tag) -> Vec<Description> {
	let mut descriptions: Vec<Description> = Vec::new();
	if let Some(description) = tag.str_values.get("Agent Description") {
		descriptions.push(Description { language: Language::English, text: description.clone() });
	}
	if let Some(description) = tag.str_values.get("Agent Description-de") {
		descriptions.push(Description { language: Language::German, text: description.clone() });
	}
	if let Some(description) = tag.str_values.get("Agent Description-es") {
		descriptions.push(Description { language: Language::Spanish, text: description.clone() });
	}
	if let Some(description) = tag.str_values.get("Agent Description-fr") {
		descriptions.push(Description { language: Language::French, text: description.clone() });
	}
	if let Some(description) = tag.str_values.get("Agent Description-it") {
		descriptions.push(Description { language: Language::Italian, text: description.clone() });
	}
	if let Some(description) = tag.str_values.get("Agent Description-nl") {
		descriptions.push(Description { language: Language::Dutch, text: description.clone() });
	}
	descriptions
}
