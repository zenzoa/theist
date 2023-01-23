use super::decode::parse_tokens;
use crate::agent::agent_tag::{ AgentTag, SupportedGame, Preview, RemoveScript };
use crate::file_helper;

pub fn encode(tag: &AgentTag) -> String {
	let mut content = String::new();

	let supported_game = match tag.supported_game {
		SupportedGame::C3 => "c3",
		SupportedGame::DS => "ds",
		SupportedGame::C3DS => "c3ds"
	};

	content.push_str(&format!("agent \"{}\" {}\n", &tag.name, supported_game));

	if !tag.version.is_empty() {
		content.push_str(&format!("\tversion \"{}\"\n", &tag.version));
	}

	if !tag.description.is_empty() {
		content.push_str(&format!("\tdescription \"{}\"\n", &tag.description.replace('"', "\\\"")));
	}

	match &tag.remove_script {
		RemoveScript::Manual(remove_script) => {
			if !remove_script.is_empty() {
				content.push_str(&format!("\tremovescript \"{}\"\n", &remove_script.replace('"', "\\\"")));
			}
		},
		RemoveScript::Auto => {
			content.push_str("\tremovescript auto\n");
		},
		_ => ()
	}

	if let Preview::Manual{ sprite, animation } = &tag.preview {
		content.push_str(&format!("\tpreview \"{}\" \"{}\"\n", &sprite, &animation));
	}

	for script in &tag.scripts {
		content.push_str(&format!("\tuse \"{}\"\n", &script));
	}

	for sprite in &tag.sprites {
		content.push_str(&format!("\tuse \"{}\"\n", &sprite));
	}

	for sound in &tag.sounds {
		content.push_str(&format!("\tuse \"{}\"\n", &sound));
	}

	for catalogue in &tag.catalogues {
		content.push_str(&format!("\tuse \"{}\"\n", &catalogue));
	}

	content.push('\n');

	content
}

pub fn decode(lines: Vec<&str>, name: String, supported_game: SupportedGame) -> (AgentTag, usize) {
	let mut version = String::new();
	let mut description = String::new();
	let mut preview = Preview::None;
	let mut remove_script = RemoveScript::None;

	let mut scripts: Vec<String> = Vec::new();
	let mut sprites: Vec<String> = Vec::new();
	let mut sounds: Vec<String> = Vec::new();
	let mut catalogues: Vec<String> = Vec::new();

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

				"description" => {
					if let Some(value) = tokens.get(1) {
						description = value.to_string();
					}
				},

				"removescript" => {
					if let Some(value) = tokens.get(1) {
						if value == "auto" {
							remove_script = RemoveScript::Auto;
						} else {
							remove_script = RemoveScript::Manual(value.to_string());
						}
					}
				},

				"preview" => {
					if let Some(value) = tokens.get(1) {
						if value == "auto" {
							preview = Preview::Auto;
						} else if let Some(value2) = tokens.get(2) {
							preview = Preview::Manual{
								sprite: value.to_string(),
								animation: value2.to_string()
							};
						}
					}
				},

				"use" => {
					let mut token_index = 1;
					while let Some(filename) = &tokens.get(token_index) {
						if filename == &"all" {
							use_all_files = true;
						} else {
							match file_helper::extension(filename).as_str() {
								"cos" => scripts.push(filename.to_string()),
								"c16" => sprites.push(filename.to_string()),
								"s16" => sprites.push(filename.to_string()),
								"blk" => sprites.push(filename.to_string()),
								"wav" => sounds.push(filename.to_string()),
								"catalogue" => catalogues.push(filename.to_string()),
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

	(AgentTag {
		name,
		version,

		description,
		supported_game,
		preview,
		remove_script,

		scripts,
		sprites,
		sounds,
		catalogues,

		use_all_files
	}, i)
}
