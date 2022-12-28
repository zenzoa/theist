use crate::agent::*;
use crate::agent::background::*;

use std::fmt;
use std::str;
use regex::Regex;
use bytes::Bytes;

#[derive(Clone)]
pub enum Tag {
	Empty,
	Agent(AgentTag),
	//Egg(EggTag)
}

impl Tag {
	pub fn set_name(&mut self, name: String) {
		match self {
			Tag::Agent(tag) => {
				tag.name = name;
			},
			_ => ()
		}
	}

	pub fn set_description(&mut self, new_description: String) {
		match self {
			Tag::Agent(tag) => {
				tag.description = new_description;
			},
			_ => ()
		}
	}

	pub fn set_version(&mut self, new_version: String) {
		match self {
			Tag::Agent(tag) => {
				tag.version = new_version;
			},
			_ => ()
		}
	}

	pub fn set_supported_game(&mut self, new_supported_game: usize) {
		match self {
			Tag::Agent(tag) => {
				tag.supported_game = match new_supported_game {
					1 => SupportedGame::C3,
					2 => SupportedGame::DS,
					_ => SupportedGame::C3DS
				};
			},
			_ => ()
		}
	}

	pub fn set_preview_auto(&mut self, is_auto: bool) {
		match self {
			Tag::Agent(tag) => {
				tag.preview = if is_auto {
					Preview::Auto
				} else {
					Preview::Manual {
						sprite: "".to_string(), // TODO: get name of first sprite
						animation: "0".to_string()
					}
				};
			},
			_ => ()
		}
	}

	pub fn set_preview_sprite(&mut self, new_sprite: String) {
		match self {
			Tag::Agent(tag) => {
				if let Preview::Manual{ animation, .. } = &tag.preview {
					tag.preview = Preview::Manual{
						sprite: new_sprite,
						animation: animation.clone()
					}
				}
			},
			_ => ()
		}
	}

	pub fn set_preview_animation(&mut self, new_animation: String) {
		match self {
			Tag::Agent(tag) => {
				if let Preview::Manual{ sprite, .. } = &tag.preview {
					tag.preview = Preview::Manual{
						sprite: sprite.clone(),
						animation: new_animation
					}
				}
			},
			_ => ()
		}
	}

	pub fn set_removescript_auto(&mut self, is_auto: bool) {
		match self {
			Tag::Agent(tag) => {
				tag.removescript = if is_auto {
					RemoveScript::Auto
				} else {
					RemoveScript::Manual("".to_string())
				};
			},
			_ => ()
		}
	}

	pub fn set_removescript_string(&mut self, new_removescript: String) {
		match self {
			Tag::Agent(tag) => {
				tag.removescript = RemoveScript::Manual(new_removescript);
			},
			_ => ()
		}
	}

	pub fn add_data(&mut self) {
		match self {
			Tag::Agent(tag) => {
				println!("Get data for agent tag \"{}\"", tag.name);

				let path = &tag.filepath;

				// script files
				for script in &tag.scripts {
					tag.script_files.push(match script.get_data(path) {
						Ok(data) => data,
						Err(_why) => Bytes::new()
					});
				}

				// sprite files
				for sprite in &tag.sprites {
					tag.sprite_files.push(match sprite.get_data(path) {
						Ok(data) => data,
						Err(_why) => Bytes::new()
					});
				}

				// background files
				for background in &mut tag.backgrounds {
					tag.background_files.push(match background.get_data(path) {
						Ok(data) => data,
						Err(_why) => Bytes::new()
					});
					*background = Background::Blk {
						filename: Filename {
							title: background.get_title(),
							extension: String::from("blk")
						}
					}
				}

				// sound files
				for sound in &tag.sounds {
					tag.sound_files.push(match sound.get_data(path) {
						Ok(data) => data,
						Err(_why) => Bytes::new()
					});
				}

				// catalogue files
				for catalogue in &tag.catalogues {
					tag.catalogue_files.push(match catalogue.get_data(path) {
						Ok(data) => data,
						Err(_why) => Bytes::new()
					});
				}

				// remove script
				if !tag.script_files.is_empty() {
					if let RemoveScript::Auto = tag.removescript {
						match str::from_utf8(&tag.script_files[0]) {
							Ok(script) => {
								let removescript_pattern = Regex::new(r"[\n\r]rscr[\n\r]([\s\S]*)").unwrap();
								match removescript_pattern.captures(script) {
									Some(captures) => {
										match captures.get(1) {
											Some(removescript) => {
												let remove_newlines_pattern = Regex::new(r"\s+").unwrap();
												let removescript = String::from(
													remove_newlines_pattern.replace_all(removescript.as_str(), " ").trim()
												);
												println!("  Remove script extracted from first script");
												tag.removescript = RemoveScript::Manual(removescript);
											},
											None => {
												println!("ERROR: No remove script found in first script.");
												tag.removescript = RemoveScript::None;
											}
										}

									},
									None => {
										println!("ERROR: No remove script found in first script.");
										tag.removescript = RemoveScript::None;
									}
								}
							},
							Err(why) => {
								println!("ERROR: Unable to extract remove script from first script: {}", why);
								tag.removescript = RemoveScript::None;
							}
						}
					}
				}

				// injector preview
				if !tag.sprites.is_empty() {
					if let Preview::Auto = tag.preview {
						let sprite_name = &tag.sprites[0].get_title();
						tag.preview = Preview::Manual {
							sprite: String::from(sprite_name),
							animation: String::from("0")
						};
						println!("  Injector preview generated");
					}
				}
			},

			Tag::Empty => ()
		}
	}
}

#[derive(Clone)]
pub struct AgentTag {
	pub filepath: String,
	pub name: String,
	pub version: String,
	pub description: String,
	pub supported_game: SupportedGame,
	pub removescript: RemoveScript,
	pub preview: Preview,

	pub scripts: Vec<Script>,
	pub sprites: Vec<Sprite>,
	pub backgrounds: Vec<Background>,
	pub sounds: Vec<Sound>,
	pub catalogues: Vec<Catalogue>,

	pub script_files: Vec<Bytes>,
	pub sprite_files: Vec<Bytes>,
	pub background_files: Vec<Bytes>,
	pub sound_files: Vec<Bytes>,
	pub catalogue_files: Vec<Bytes>
}

impl AgentTag {
	pub fn new() -> AgentTag {
		AgentTag {
			filepath: String::from(""),
			name: String::from(""),
			version: String::from(""),
			description: String::from(""),
			supported_game: SupportedGame::C3DS,
			removescript: RemoveScript::Auto,
			preview: Preview::Auto,

			scripts: Vec::new(),
			sprites: Vec::new(),
			backgrounds: Vec::new(),
			sounds: Vec::new(),
			catalogues: Vec::new(),

			script_files: Vec::new(),
			sprite_files: Vec::new(),
			background_files: Vec::new(),
			sound_files: Vec::new(),
			catalogue_files: Vec::new()
		}
	}
}

//pub struct EggTag {
//	filepath: String,
//	name: String,
//	version: String,
//	preview_sprite_male: String,
//	preview_sprite_female: String,
//	preview_animation: String,
//	genetics: Vec<Genetics>,
//	sprites: Vec<Sprite>
//}

#[derive(Clone, PartialEq)]
pub enum RemoveScript {
	None,
	Auto,
	Manual(String)
}

impl fmt::Display for RemoveScript {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match &self {
			RemoveScript::None => write!(f, ""),
			RemoveScript::Auto => write!(f, "auto"),
			RemoveScript::Manual(s) => write!(f, "{}", s),
		}
	}
}

#[derive(Clone, PartialEq)]
pub enum Preview {
	Auto,
	Manual { sprite: String, animation: String }
}
