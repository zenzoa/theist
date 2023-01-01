use crate::agent::*;
use crate::agent::background::*;
use crate::agent::agent_tag::*;
use crate::agent::egg_tag::*;

use std::str;
use regex::Regex;
use bytes::Bytes;

#[derive(Clone)]
pub enum Tag {
	Empty,
	Agent(AgentTag),
	Egg(EggTag)
}

impl Tag {
	pub fn set_name(&mut self, new_name: String) {
		match self {
			Tag::Agent(tag) => {
				tag.name = new_name;
			},
			Tag::Egg(tag) => {
				tag.name = new_name;
			},
			Tag::Empty => ()
		}
	}

	pub fn set_description(&mut self, new_description: String) {
		if let Tag::Agent(tag) = self {
			tag.description = new_description;
		}
	}

	pub fn set_version(&mut self, new_version: String) {
		match self {
			Tag::Agent(tag) => {
				tag.version = new_version;
			},
			Tag::Egg(tag) => {
				tag.version = new_version;
			},
			Tag::Empty => ()
		}
	}

	pub fn set_supported_game(&mut self, new_supported_game: usize) {
		if let Tag::Agent(tag) = self {
			tag.supported_game = match new_supported_game {
				1 => SupportedGame::C3,
				2 => SupportedGame::DS,
				_ => SupportedGame::C3DS
			};
		}
	}

	pub fn set_preview_auto(&mut self, is_auto: bool) {
		if let Tag::Agent(tag) = self {
			tag.preview = if is_auto {
				Preview::Auto
			} else {
				let first_sprite_name = if let Some(sprite) = tag.sprites.get(0) {
					match sprite {
						Sprite::C16{ filename, .. } => filename.title.clone(),
						Sprite::Frames{ filename, .. } => filename.title.clone()
					}
				} else {
					String::from("")
				};
				Preview::Manual {
					sprite: first_sprite_name,
					animation: "0".to_string()
				}
			};
		}
	}

	pub fn set_preview_sprite(&mut self, new_sprite: String) {
		if let Tag::Agent(tag) = self {
			if let Preview::Manual{ animation, .. } = &tag.preview {
				tag.preview = Preview::Manual{
					sprite: new_sprite,
					animation: animation.clone()
				}
			}
		}
	}

	pub fn set_preview_animation(&mut self, new_animation: String) {
		if let Tag::Agent(tag) = self {
			if let Preview::Manual{ sprite, .. } = &tag.preview {
				tag.preview = Preview::Manual{
					sprite: sprite.clone(),
					animation: new_animation
				}
			}
		}
	}

	pub fn set_removescript_auto(&mut self, is_auto: bool) {
		if let Tag::Agent(tag) = self {
			tag.removescript = if is_auto {
				RemoveScript::Auto
			} else {
				RemoveScript::Manual("".to_string())
			};
		}
	}

	pub fn set_removescript_string(&mut self, new_removescript: String) {
		if let Tag::Agent(tag) = self {
			tag.removescript = RemoveScript::Manual(new_removescript);
		}
	}

	pub fn add_data(&mut self) {
		match self {
			Tag::Agent(tag) => {
				println!("Get data for agent tag \"{}\"", tag.name);

				let path = &tag.filepath;

				// script files
				for script in tag.scripts.iter_mut() {
					tag.script_files.push(match script.get_data(path) {
						Ok(data) => data,
						Err(_why) => Bytes::new()
					});
				}

				// sprite files
				for sprite in tag.sprites.iter_mut() {
					tag.sprite_files.push(match sprite.get_data(path) {
						Ok(data) => data,
						Err(_why) => Bytes::new()
					});
				}

				// background files
				for background in tag.backgrounds.iter_mut() {
					tag.background_files.push(match background.get_data(path) {
						Ok(data) => data,
						Err(_why) => Bytes::new()
					});
					*background = Background::Blk {
						filename: Filename::new(format!("{}.blk", background.get_title()).as_str())
					}
				}

				// sound files
				for sound in tag.sounds.iter_mut() {
					tag.sound_files.push(match sound.get_data(path) {
						Ok(data) => data,
						Err(_why) => Bytes::new()
					});
				}

				// catalogue files
				for catalogue in tag.catalogues.iter_mut() {
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
						let sprite_name = &tag.sprites.get(0).unwrap().get_title();
						tag.preview = Preview::Manual {
							sprite: String::from(sprite_name),
							animation: String::from("0")
						};
						println!("  Injector preview generated");
					}
				}
			},

			Tag::Egg(_tag) => (),

			Tag::Empty => ()
		}
	}
}
