use super::decode::parse_tokens;
use crate::agent::egg_tag::EggTag;
use crate::file_helper;

pub fn encode(tag: &EggTag) -> String {
	let mut content = String::new();

	content.push_str(&format!("egg \"{}\"\n", &tag.name));

	content.push_str(&format!("\tpreview \"{}\" \"{}\" \"{}\"\n",
		&tag.preview_sprite_male,
		&tag.preview_sprite_female,
		&tag.preview_animation));

	content.push_str(&format!("\tgenome \"{}\"\n", &tag.genome));

	for sprite in &tag.sprites {
		content.push_str(&format!("\tsprite \"{}\"\n", &sprite));
	}

	for bodydata in &tag.bodydata {
		content.push_str(&format!("\tbodydata \"{}\"\n", &bodydata));
	}

	content.push('\n');

	content
}

pub fn decode(lines: Vec<&str>, name: String) -> (EggTag, usize) {
	let mut version = String::new();
	let mut preview_sprite_male = String::new();
	let mut preview_sprite_female = String::new();
	let mut preview_animation = String::new();
	let mut genome = String::new();

	let mut sprites: Vec<String> = Vec::new();
	let mut bodydata: Vec<String> = Vec::new();

	let mut use_all_files = false;

	let mut i = 0;
	while i < lines.len() {
		let tokens = parse_tokens(lines[i]);

		if let Some(token) = tokens.get(0) {
			match token.as_str() {
				"version" => {
					if let Some(value) = tokens.get(1) {
						version = value.to_string();
					}
				},

				"preview" => {
					if let Some(value) = tokens.get(1) {
						if let Some(value2) = tokens.get(2) {
							if let Some(value3) = tokens.get(3) {
								preview_sprite_male = value.to_string();
								preview_sprite_female = value2.to_string();
								preview_animation = value3.to_string();
							}
						}
					}
				},

				"genome" => {
					if let Some(filename) = tokens.get(1) {
						genome = filename.to_string();
					}
				},

				"use" => {
					let mut token_index = 1;
					while let Some(filename) = &tokens.get(token_index) {
						if filename == &"all" {
							use_all_files = true;
						} else {
							match file_helper::extension(filename).as_str() {
								"c16" => sprites.push(filename.to_string()),
								"s16" => sprites.push(filename.to_string()),
								"att" => bodydata.push(filename.to_string()),
								_ => ()
							}
						}
						token_index += 1;
					}
				},

				_ => { break; }
			}
		} else {
			break;
		}

		i += 1;
	}

	(EggTag {
		name,
		version,

		preview_sprite_male,
		preview_sprite_female,
		preview_animation,
		genome,

		sprites,
		bodydata,

		use_all_files
	}, i)
}
