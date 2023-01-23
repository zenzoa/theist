use super::decode::parse_tokens;
use crate::agent::file::CreaturesFile;
use crate::agent::script::Script;
use crate::agent::sprite::{ Sprite, SpriteFrame };
use crate::agent::sound::Sound;
use crate::agent::catalogue::{ Catalogue, CatalogueEntry };
use crate::agent::genetics::Genetics;
use crate::agent::bodydata::BodyData;

use std::error::Error;

pub fn encode(file: &CreaturesFile) -> Result<String, Box<dyn Error>> {
	let mut content = String::new();
	match file {
		CreaturesFile::Script(script) => {
			content.push_str(&format!("\tscript \"{}\"\n", &script.get_output_filename()));
			if script.input_filename != script.get_output_filename() {
				content.push_str(&format!("\t\tsource \"{}\"\n", &script.input_filename));
			}
		},

		CreaturesFile::Sprite(sprite) => {
			content.push_str(&format!("\tsprite \"{}\"\n", &sprite.get_output_filename()));
			match sprite {
				Sprite::Raw{ input_filename, .. } => {
					if input_filename != &sprite.get_output_filename() {
						content.push_str(&format!("\t\tsource \"{}\"\n", &input_filename));
					}
				},
				Sprite::Png{ frames, .. } => {
					for frame in frames {
						content.push_str(&format!("\t\tframe \"{}\"\n", &frame.input_filename));
					}
				}
			}
		},

		CreaturesFile::Sound(sound) => {
			content.push_str(&format!("\tsound \"{}\"\n", &sound.get_output_filename()));
			if sound.input_filename != sound.get_output_filename() {
				content.push_str(&format!("\t\tsource \"{}\"\n", &sound.input_filename));
			}
		},

		CreaturesFile::Catalogue(catalogue) => {
			content.push_str(&format!("\tcatalogue \"{}\"\n", &catalogue.get_output_filename()));
			match catalogue {
				Catalogue::Raw{ input_filename, .. } => {
					if input_filename != &catalogue.get_output_filename() {
						content.push_str(&format!("\t\tsource \"{}\"\n", &input_filename));
					}
				},
				Catalogue::Inline{ entries, .. } => {
					for entry in entries {
						let description = &entry.description.replace('"', "\\\"");
						content.push_str(&format!("\t\tentry \"{}\" \"{}\" \"{}\"\n", &entry.classifier, &entry.name, description));
					}
				}
			}
		},

		CreaturesFile::BodyData(bodydata) => {
			content.push_str(&format!("\tbodydata \"{}\"\n", &bodydata.get_output_filename()));
			if bodydata.input_filename != bodydata.get_output_filename() {
				content.push_str(&format!("\t\tsource \"{}\"\n", &bodydata.input_filename));
			}
		},

		CreaturesFile::Genetics(genetics) => {
			content.push_str(&format!("\tgenome \"{}\"\n", &genetics.get_output_filename()));
			if genetics.input_filename != genetics.get_output_filename() {
				content.push_str(&format!("\t\tsource \"{}\"\n", &genetics.input_filename));
			}
		}
	}
	Ok(content)
}

pub fn decode(lines: Vec<&str>) -> (Vec<CreaturesFile>, usize) {
	let mut files: Vec<CreaturesFile> = Vec::new();

	let mut i = 0;
	while i < lines.len() {
		let tokens = parse_tokens(lines[i]);

		if let Some(token) = tokens.get(0) {
			match token.as_str() {
				"script" => {
					if let Some(filename) = tokens.get(1) {
						files.push(CreaturesFile::Script(Script{
							output_filename: filename.to_string(),
							input_filename: filename.to_string(),
							data: None
						}));
					}
				},

				"sprite" => {
					if let Some(filename) = tokens.get(1) {
						files.push(CreaturesFile::Sprite(Sprite::Raw{
							output_filename: filename.to_string(),
							input_filename: filename.to_string(),
							data: None
						}));
					}
				},

				"sound" => {
					if let Some(filename) = tokens.get(1) {
						files.push(CreaturesFile::Sound(Sound{
							output_filename: filename.to_string(),
							input_filename: filename.to_string(),
							data: None
						}));
					}
				},

				"catalogue" => {
					if let Some(filename) = tokens.get(1) {
						files.push(CreaturesFile::Catalogue(Catalogue::Raw{
							output_filename: filename.to_string(),
							input_filename: filename.to_string(),
							data: None
						}));
					}
				},

				"genome" => {
					if let Some(filename) = tokens.get(1) {
						files.push(CreaturesFile::Genetics(Genetics{
							output_filename: filename.to_string(),
							input_filename: filename.to_string(),
							data: None
						}));
					}
				},

				"bodydata" => {
					if let Some(filename) = tokens.get(1) {
						files.push(CreaturesFile::BodyData(BodyData{
							output_filename: filename.to_string(),
							input_filename: filename.to_string(),
							data: None
						}));
					}
				},

				"frame" => {
					if let Some(input_filename) = tokens.get(1) {
						if let Some(CreaturesFile::Sprite(sprite)) = files.last_mut() {
							if let Sprite::Raw{ output_filename, .. } = sprite {
								*sprite = Sprite::Png{
									output_filename: output_filename.clone(),
									frames: Vec::new(),
									data: None
								}
							}
							sprite.add_frame(SpriteFrame{
								input_filename: input_filename.to_string(),
								data: None
							});
						}
					}
				},

				"entry" => {
					if let Some(classifier) = tokens.get(1) {
						if let Some(name) = tokens.get(2) {
							if let Some(description) = tokens.get(3) {
								if let Some(CreaturesFile::Catalogue(catalogue)) = files.last_mut() {
									if let Catalogue::Raw{ output_filename, .. } = catalogue {
										*catalogue = Catalogue::Inline{
											output_filename: output_filename.clone(),
											entries: Vec::new(),
											data: None
										}
									}
									catalogue.add_entry(CatalogueEntry{
										classifier: classifier.to_string(),
										name: name.to_string(),
										description: description.to_string()
									});
								}
							}
						}
					}
				},

				"source" => {
					if let Some(value) = tokens.get(1) {
						let input_filename = value.to_string();
						if let Some(file) = &mut files.last_mut() {
							match file {
								CreaturesFile::Script(script) => {
									script.input_filename = input_filename;
								},

								CreaturesFile::Sprite(sprite) => {
									if let Sprite::Raw{ output_filename, .. } = sprite {
										*sprite = Sprite::Png{
											output_filename: output_filename.clone(),
											frames: vec![SpriteFrame{
												input_filename: input_filename.to_string(),
												data: None
											}],
											data: None
										}
									}
								},

								CreaturesFile::Sound(sound) => {
									sound.input_filename = input_filename;
								},

								CreaturesFile::Catalogue(catalogue) => {
									if let Catalogue::Raw{ input_filename: og_input_filename, .. } = catalogue {
										*og_input_filename = input_filename;
									}
								},

								CreaturesFile::Genetics(genetics) => {
									genetics.input_filename = input_filename;
								},

								CreaturesFile::BodyData(bodydata) => {
									bodydata.input_filename = input_filename;
								}
							}
						}
					}
				},

				_ => { break; }
			}
		} else {
			break;
		}

		i += 1;
	}

	(files, i)
}
