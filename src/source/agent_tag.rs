use super::decode::parse_tokens;
use crate::agent::file::{ CreaturesFile, FileType, lookup_file_index };
use crate::agent::agent_tag::{ AgentTag, SupportedGame, Preview, RemoveScript };

pub fn encode(tag: &AgentTag, files: &[CreaturesFile]) -> String {
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
		if let Some(CreaturesFile::Sprite(sprite_file)) = files.get(sprite.clone()) {
			content.push_str(&format!("\tpreview \"{}\" \"{}\"\n", sprite_file.get_output_filename(), &animation));
		}
	}

	if tag.scripts.len() + tag.sprites.len() + tag.sounds.len() + tag.catalogues.len() == files.len() {
		content.push_str("\tuse all");

	} else {
		for script in &tag.scripts {
			if let Some(CreaturesFile::Script(script_file)) = files.get(script.clone()) {
				content.push_str(&format!("\tuse \"{}\"\n", script_file.get_output_filename()));
			}
		}

		for sprite in &tag.sprites {
			if let Some(CreaturesFile::Sprite(sprite_file)) = files.get(sprite.clone()) {
				content.push_str(&format!("\tuse \"{}\"\n", sprite_file.get_output_filename()));
			}
		}

		for sound in &tag.sounds {
			if let Some(CreaturesFile::Sound(sound_file)) = files.get(sound.clone()) {
				content.push_str(&format!("\tuse \"{}\"\n", sound_file.get_output_filename()));
			}
		}

		for catalogue in &tag.catalogues {
			if let Some(CreaturesFile::Catalogue(catalogue_file)) = files.get(catalogue.clone()) {
				content.push_str(&format!("\tuse \"{}\"\n", catalogue_file.get_output_filename()));
			}
		}
	}

	content.push('\n');

	content
}

pub fn decode(lines: Vec<&str>, name: String, supported_game: SupportedGame, files: &[CreaturesFile]) -> (AgentTag, usize) {
	let mut version = String::new();
	let mut description = String::new();
	let mut preview = Preview::None;
	let mut remove_script = RemoveScript::None;

	let mut scripts: Vec<usize> = Vec::new();
	let mut sprites: Vec<usize> = Vec::new();
	let mut sounds: Vec<usize> = Vec::new();
	let mut catalogues: Vec<usize> = Vec::new();

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
							if let Some(sprite_index) = lookup_file_index(files, value) {
								preview = Preview::Manual{
									sprite: sprite_index,
									animation: value2.to_string()
								};
							}
						}
					}
				},

				"use" => {
					let mut token_index = 1;
					while let Some(filename) = &tokens.get(token_index) {
						if filename == &"all" {
							use_all_files = true;
						} else {
							if let Some(file_index) = lookup_file_index(files, filename) {
								match files[file_index].get_filetype() {
									FileType::Script => scripts.push(file_index),
									FileType::Sprite => sprites.push(file_index),
									FileType::Sound => sounds.push(file_index),
									FileType::Catalogue => catalogues.push(file_index),
									_ => ()
								}
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

		remove_script_backup: RemoveScript::None,
		preview_backup: Preview::None,

		scripts,
		sprites,
		sounds,
		catalogues,

		use_all_files
	}, i)
}
