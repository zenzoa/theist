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

	pub fn delete_script(&mut self, index: usize) {
		if let Tag::Agent(tag) = self {
			if index < tag.scripts.len() {
				tag.scripts.remove(index);
			}
		}
	}
	pub fn move_script_up(&mut self, index: usize) {
		if let Tag::Agent(tag) = self {
			if index > 0 && index < tag.scripts.len() {
				tag.scripts.swap(index, index - 1);
			}
		}
	}
	pub fn move_script_down(&mut self, index: usize) {
		if let Tag::Agent(tag) = self {
			if index + 1 < tag.scripts.len() {
				tag.scripts.swap(index, index + 1);
			}
		}
	}
	pub fn set_script_supported_game(&mut self, index: usize, new_supported_game: usize) {
		if let Tag::Agent(tag) = self {
			if let Some(Script::File{ supported_game, .. }) = tag.scripts.get_mut(index) {
				*supported_game = match new_supported_game {
					1 => SupportedGame::C3,
					2 => SupportedGame::DS,
					_ => SupportedGame::C3DS
				};
			}
		}
	}

	pub fn delete_sprite(&mut self, index: usize) {
		if let Tag::Agent(tag) = self {
			if index < tag.sprites.len() {
				tag.sprites.remove(index);
			}
		}
	}
	pub fn move_sprite_up(&mut self, index: usize) {
		if let Tag::Agent(tag) = self {
			if index > 0 && index < tag.sprites.len() {
				tag.sprites.swap(index, index - 1);
			}
		}
	}
	pub fn move_sprite_down(&mut self, index: usize) {
		if let Tag::Agent(tag) = self {
			if index + 1 < tag.sprites.len() {
				tag.sprites.swap(index, index + 1);
			}
		}
	}
	pub fn set_sprite_name(&mut self, index: usize, new_name: String) {
		if let Tag::Agent(tag) = self {
			if let Some(Sprite::Frames{ filename, .. }) = tag.sprites.get_mut(index) {
				filename.set_title(new_name);
			}
		}
	}
	pub fn convert_sprite_to_background(&mut self, index: usize) -> Option<usize> {
		if let Tag::Agent(tag) = self {
			if let Some(Sprite::Frames{ frames, .. }) = tag.sprites.get(index) {
				if let Some(frame) = frames.get(0) {
					tag.backgrounds.push(Background::new(frame.filename.to_string().as_str()));
					let background_index = &tag.backgrounds.len() - 1;
					self.delete_sprite(index);
					return Some(background_index);
				}
			}
		}
		None
	}

	pub fn add_sprite_frame(&mut self, index: usize, filename: String) {
		if let Tag::Agent(tag) = self {
			if let Some(sprite) = tag.sprites.get_mut(index) {
				sprite.add_frame(filename.as_str());
			}
		}
	}
	pub fn delete_sprite_frame(&mut self, sprite_index: usize, frame_index: usize) {
		if let Tag::Agent(tag) = self {
			if let Some(Sprite::Frames{ frames, .. }) = tag.sprites.get_mut(sprite_index) {
				if frame_index < frames.len() {
					frames.remove(frame_index);
				}
			}
		}
	}
	pub fn move_sprite_frame_up(&mut self, sprite_index: usize, frame_index: usize) {
		if let Tag::Agent(tag) = self {
			if let Some(Sprite::Frames{ frames, .. }) = tag.sprites.get_mut(sprite_index) {
				if frame_index > 0 && frame_index < frames.len() {
					frames.swap(frame_index, frame_index - 1);
				}
			}
		}
	}
	pub fn move_sprite_frame_down(&mut self, sprite_index: usize, frame_index: usize) {
		if let Tag::Agent(tag) = self {
			if let Some(Sprite::Frames{ frames, .. }) = tag.sprites.get_mut(sprite_index) {
				if frame_index + 1 < frames.len() {
					frames.swap(frame_index, frame_index + 1);
				}
			}
		}
	}

	pub fn delete_background(&mut self, index: usize) {
		if let Tag::Agent(tag) = self {
			if index < tag.backgrounds.len() {
				tag.backgrounds.remove(index);
			}
		}
	}
	pub fn move_background_up(&mut self, index: usize) {
		if let Tag::Agent(tag) = self {
			if index > 0 && index < tag.backgrounds.len() {
				tag.backgrounds.swap(index, index - 1);
			}
		}
	}
	pub fn move_background_down(&mut self, index: usize) {
		if let Tag::Agent(tag) = self {
			if index + 1 < tag.backgrounds.len() {
				tag.backgrounds.swap(index, index + 1);
			}
		}
	}
	pub fn convert_background_to_sprite(&mut self, index: usize) -> Option<usize> {
		if let Tag::Agent(tag) = self {
			if let Some(Background::Png{ source, .. }) = tag.backgrounds.get(index) {
				let mut new_sprite = Sprite::new(format!("{}.c16", &source.title).as_str());
				new_sprite.add_frame(source.to_string().as_str());
				tag.sprites.push(new_sprite);
				let sprite_index = &tag.sprites.len() - 1;
				self.delete_background(index);
				return Some(sprite_index);
			}
		}
		None
	}

	pub fn delete_sound(&mut self, index: usize) {
		if let Tag::Agent(tag) = self {
			if index < tag.sounds.len() {
				tag.sounds.remove(index);
			}
		}
	}
	pub fn move_sound_up(&mut self, index: usize) {
		if let Tag::Agent(tag) = self {
			if index > 0 && index < tag.sounds.len() {
				tag.sounds.swap(index, index - 1);
			}
		}
	}
	pub fn move_sound_down(&mut self, index: usize) {
		if let Tag::Agent(tag) = self {
			if index + 1 < tag.sounds.len() {
				tag.sounds.swap(index, index + 1);
			}
		}
	}

	pub fn add_inline_catalogue(&mut self) {
		if let Tag::Agent(tag) = self {
			tag.catalogues.push(
				Catalogue::Inline{
					filename: Filename::new("my_catalogue.catalogue"),
					entries: vec![
						CatalogueEntry::new("0 0 0000", tag.name.as_str(), tag.description.as_str())
					]
				}
			)
		}
	}
	pub fn delete_catalogue(&mut self, index: usize) {
		if let Tag::Agent(tag) = self {
			if index < tag.catalogues.len() {
				tag.catalogues.remove(index);
			}
		}
	}
	pub fn move_catalogue_up(&mut self, index: usize) {
		if let Tag::Agent(tag) = self {
			if index > 0 && index < tag.catalogues.len() {
				tag.catalogues.swap(index, index - 1);
			}
		}
	}
	pub fn move_catalogue_down(&mut self, index: usize) {
		if let Tag::Agent(tag) = self {
			if index + 1 < tag.catalogues.len() {
				tag.catalogues.swap(index, index + 1);
			}
		}
	}
	pub fn set_catalogue_name(&mut self, index: usize, new_name: String) {
		if let Tag::Agent(tag) = self {
			if let Some(Catalogue::Inline{ filename, .. }) = tag.catalogues.get_mut(index) {
				filename.set_title(new_name);
			}
		}
	}

	pub fn add_catalogue_entry(&mut self, index: usize) {
		if let Tag::Agent(tag) = self {
			if let Some(catalogue) = tag.catalogues.get_mut(index) {
				catalogue.add_entry(
					CatalogueEntry::new("0 0 0000", tag.name.as_str(), tag.description.as_str())
				);
			}
		}
	}
	pub fn delete_catalogue_entry(&mut self, catalogue_index: usize, entry_index: usize) {
		if let Tag::Agent(tag) = self {
			if let Some(Catalogue::Inline{ entries, .. }) = tag.catalogues.get_mut(catalogue_index) {
				if entry_index < entries.len() {
					entries.remove(entry_index);
				}
			}
		}
	}
	pub fn move_catalogue_entry_up(&mut self, catalogue_index: usize, entry_index: usize) {
		if let Tag::Agent(tag) = self {
			if let Some(Catalogue::Inline{ entries, .. }) = tag.catalogues.get_mut(catalogue_index) {
				if entry_index > 0 && entry_index < entries.len() {
					entries.swap(entry_index, entry_index - 1);
				}
			}
		}
	}
	pub fn move_catalogue_entry_down(&mut self, catalogue_index: usize, entry_index: usize) {
		if let Tag::Agent(tag) = self {
			if let Some(Catalogue::Inline{ entries, .. }) = tag.catalogues.get_mut(catalogue_index) {
				if entry_index + 1 < entries.len() {
					entries.swap(entry_index, entry_index + 1);
				}
			}
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
						filename: Filename::new(format!("{}.blk", background.get_title()).as_str())
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

			Tag::Egg(_tag) => (),

			Tag::Empty => ()
		}
	}
}
