use super::decode::parse_tokens;
use crate::agent::file::{ CreaturesFile, FileType, lookup_file_index };
use crate::agent::egg_tag::{ EggTag, EggPreview };

pub fn encode(tag: &EggTag, files: &[CreaturesFile]) -> String {
	let mut content = String::new();

	content.push_str(&format!("egg \"{}\"\n", &tag.name));

	if let EggPreview::Manual{ sprite_male, sprite_female, animation } = &tag.preview {
		if let Some(sprite_male_file) = files.get(*sprite_male) {
			if let Some(sprite_female_file) = files.get(*sprite_female) {
				content.push_str(&format!("\tpreview \"{}\" \"{}\" \"{}\"\n",
					sprite_male_file.get_output_filename(),
					sprite_female_file.get_output_filename(),
					animation));
			}
		}
	}

	if let Some(genome) = tag.genome {
		if let Some(CreaturesFile::Genetics(genome_file)) = files.get(genome) {
			content.push_str(&format!("\tgenome \"{}\"\n", genome_file.get_title()));
		}
	}

	for sprite in &tag.sprites {
		if let Some(CreaturesFile::Sprite(sprite_file)) = files.get(*sprite) {
			content.push_str(&format!("\tuse \"{}\"\n", sprite_file.get_output_filename()));
		}
	}

	for bodydata in &tag.bodydata {
		if let Some(CreaturesFile::BodyData(bodydata_file)) = files.get(*bodydata) {
			content.push_str(&format!("\tuse \"{}\"\n", bodydata_file.get_output_filename()));
		}
	}

	content.push('\n');

	content
}

pub fn decode(lines: Vec<&str>, name: String, files: &[CreaturesFile]) -> (EggTag, usize) {
	let mut version = String::new();
	let mut preview = EggPreview::None;
	let mut genome = None;

	let mut sprites: Vec<usize> = Vec::new();
	let mut bodydata: Vec<usize> = Vec::new();
	let mut genetics: Vec<usize> = Vec::new();

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
					if let Some(sprite_male) = tokens.get(1) {
						if let Some(sprite_female) = tokens.get(2) {
							if let Some(animation) = tokens.get(3) {
								if let Some(sprite_male_index) = lookup_file_index(files, sprite_male) {
									if let Some(sprite_female_index) = lookup_file_index(files, sprite_female) {
										preview = EggPreview::Manual{
											sprite_male: sprite_male_index,
											sprite_female: sprite_female_index,
											animation: animation.to_string()
										}
									}
								}
							}
						}
					}
				},

				"genome" => {
					if let Some(title) = tokens.get(1) {
						let gen_filename = format!("{}.gen", title);
						let gno_filename = format!("{}.gno", title);
						if let Some(file_index) = lookup_file_index(files, &gen_filename) {
							genome = Some(file_index);
							genetics.push(file_index);
						}
						if let Some(file_index) = lookup_file_index(files, &gno_filename) {
							genetics.push(file_index);
						}
					}
				},

				"use" => {
					let mut token_index = 1;
					while let Some(filename) = &tokens.get(token_index) {
						if filename == &"all" {
							use_all_files = true;
						} else if let Some(file_index) = lookup_file_index(files, filename) {
							match files[file_index].get_filetype() {
								FileType::Sprite => sprites.push(file_index),
								FileType::BodyData => bodydata.push(file_index),
								FileType::Genetics => genetics.push(file_index),
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

		preview,
		genome,

		preview_backup: EggPreview::None,

		sprites,
		bodydata,
		genetics,

		use_all_files
	}, i)
}
