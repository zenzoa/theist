pub mod tag;
pub mod agent_tag;
pub mod egg_tag;
pub mod script;
pub mod sprite;
pub mod background;
pub mod sound;
pub mod catalogue;

use tag::*;
use agent_tag::*;
use script::*;
use sprite::*;
use background::*;
use sound::*;
use catalogue::*;

use crate::pray;

use std::fmt;
use std::str;
use std::error::Error;
use regex::Regex;
use bytes::Bytes;

pub struct FileData {
	pub name: String,
	pub data: Bytes
}

#[derive(Clone)]
pub struct Filename {
	pub string: String,
	pub title: String,
	pub extension: String
}

impl Filename {
	pub fn new(filename_string: &str) -> Filename {
		let filename_pattern = Regex::new(r"^(.+)\.(.+)$").unwrap();
		match filename_pattern.captures(filename_string) {
			None => Filename {
				string: String::from(""),
				title: String::from(""),
				extension: String::from("")
			},
			Some(captures) => Filename {
				string: String::from(filename_string),
				title: String::from(&captures[1]),
				extension: String::from(&captures[2])
			}
		}
	}

	pub fn set_title(&mut self, new_title: String) {
		self.title = new_title;
		self.string = format!("{}.{}", &self.title, &self.extension);
	}

	pub fn with_extension(&self, new_extension: &str) -> Filename {
		Filename {
			string: format!("{}.{}", &self.title, &new_extension),
			title: self.title.clone(),
			extension: new_extension.to_string()
		}
	}
}

impl fmt::Display for Filename {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", &self.string)
	}
}

#[derive(Clone)]
pub enum SupportedGame {
	C3,
	DS,
	C3DS
}

impl fmt::Display for SupportedGame {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match &self {
			SupportedGame::C3 => write!(f, "c3"),
			SupportedGame::DS => write!(f, "ds"),
			SupportedGame::C3DS => write!(f, "c3ds"),
		}
	}
}

impl SupportedGame {
	pub fn new(s: &str) -> SupportedGame {
		match s {
			"c3" => SupportedGame::C3,
			"C3" => SupportedGame::C3,
			"ds" => SupportedGame::DS,
			"DS" => SupportedGame::DS,
			_ => SupportedGame::C3DS
		}
	}
}

fn parse_tokens(s: &str) -> Vec<String> {
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
				'\'' => {
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

pub fn parse_source(contents: &str, path: &str) -> Vec<Tag> {
	//let contents = "agent \"Aibo Ball\" c3ds\n\nversion \"1.0.4\"\ndescription \"We took the iconic pink ball away from an aibo so your norns can play with it instead.\"\npreview \"aibo_ball.c16\" \"1 2 3 3\"\nremovescript \"enum 000\"\nscript \"aibo_ball.cos\" ds\nsprite \"aibo_ball.c16\"\nframe \"aibo_ball1.png\"\nframe \"aibo_ball2.png\"\nframe \"aibo_ball3.png\"\nbackground \"bg.png\"\nsound \"blop.wav\"\ncatalogue \"aibo_ball.catalogue\"\nentry \"2 21 21212\" \"Aibo Ball\" \"We took the iconic pink ball away from an aibo so your norns can play with it instead.\"";
	let mut tags: Vec<Tag> = Vec::new();

	for line in contents.lines() {
		let tokens = parse_tokens(line.trim());
		if tokens.is_empty() {
			continue;
		}
		let token = tokens.get(0).unwrap().as_str();

		let num_tags = tags.len();
		let mut empty_tag = Tag::Empty;
		let current_tag = if num_tags > 0 { &mut tags[num_tags - 1] } else { &mut empty_tag };

		match current_tag {
			Tag::Agent(tag) => {
				match token {
					"version" => {
						if let Some(i) = tokens.get(1) {
							tag.version = String::from(i);
						}
						println!("  Version: {}", tag.version);
					},
					"description" => {
						if let Some(i) = tokens.get(1) {
							tag.description = String::from(i);
						}
						println!("  Description: {}", tag.description);
					},
					"preview" => {
						// TODO: check for "preview auto"
						let sprite = match tokens.get(1) {
							None => String::from(""),
							Some(i) => String::from(i)
						};
						let animation = match tokens.get(2) {
							None => String::from("0"),
							Some(i) => String::from(i)
						};
						if !sprite.is_empty() {
							println!("  Preview: {} \"{}\"", sprite, animation);
							tag.preview = Preview::Manual{ sprite, animation };
						}
					},
					"removescript" => {
						tag.removescript = match tokens.get(1) {
							None => RemoveScript::None,
							Some(i) => match i.as_str() {
								"auto" => RemoveScript::Auto,
								_ => RemoveScript::Manual(i.to_string())
							}
						};
						println!("  Remove script: {}", tag.removescript)
					},
					"script" => {
						if let Some(filename) = tokens.get(1) {
							let supported_game = match tokens.get(2) {
								None => "c3ds",
								Some(i) => i.as_str()
							};
							let script = Script::new(filename, supported_game);
							tag.scripts.push(script);
							println!("  Add script (total: {})", tag.scripts.len());
						}
					},
					"sprite" => {
						if let Some(filename) = tokens.get(1) {
							let sprite = Sprite::new(filename);
							tag.sprites.push(sprite);
							println!("  Add sprite (total: {})", tag.scripts.len());
						}
					},
					"frame" => {
						let num_sprites = tag.sprites.len();
						if let Some(current_sprite) = tag.sprites.get_mut(num_sprites - 1) {
							if let Some(filename) = tokens.get(1) {
								current_sprite.add_frame(filename);
								if let Sprite::Frames { frames, .. } = current_sprite {
									println!("    Add frame (total: {})", frames.len());
								}
							}
						}
					},
					"background" => {
						if let Some(filename) = tokens.get(1) {
							let background = Background::new(filename);
							tag.backgrounds.push(background);
							println!("  Add background (total: {})", tag.backgrounds.len());
						}
					},
					"sound" => {
						if let Some(filename) = tokens.get(1) {
							let sound = Sound::new(filename);
							tag.sounds.push(sound);
							println!("  Add sound (total: {})", tag.sounds.len());
						}
					},
					"catalogue" => {
						if let Some(filename) = tokens.get(1) {
							let catalogue = Catalogue::new(filename);
							tag.catalogues.push(catalogue);
							println!("  Add catalogue (total: {})", tag.catalogues.len());
						}
					},
					"entry" => {
						let num_catalogues = tag.catalogues.len();
						if let Some(current_catalogue) = tag.catalogues.get_mut(num_catalogues - 1) {
							if let (Some(classifier), Some(name), Some(description)) = (tokens.get(1), tokens.get(2), tokens.get(3)) {
								let entry = CatalogueEntry::new(classifier, name, description);
								current_catalogue.add_entry(entry);
								if let Catalogue::Inline { entries, .. } = current_catalogue {
									println!("    Add entry (total: {})", entries.len());
								}
							}
						}
					},
					_ => ()
				}
			},
			_ => {
				match token {
					"agent" => {
						let mut tag = AgentTag::new(String::from(""));
						if let Some(i) = tokens.get(1) {
							tag.name = String::from(i);
						}
						if let Some(i) = tokens.get(2) {
							tag.supported_game = SupportedGame::new(i.as_str());
						}
						tag.filepath = String::from(path);
						println!("Add agent \"{}\"", tag.name);
						println!("  Path: {}", tag.filepath);
						println!("  Supported game: {}", tag.supported_game);
						tags.push(Tag::Agent(tag));
					},
					"egg" => {
						println!("Add egg");
					},
					_ => ()
				}
			}
		}
	}

	tags
}

pub fn encode_source(tags: Vec<Tag>) -> Bytes {
	let mut source = String::from("");

	for tag in tags {
		if let Tag::Agent(tag) = tag {
			source += format!("agent \"{}\" {}\n", &tag.name, &tag.supported_game).as_str();

			if !tag.description.is_empty() {
				source += format!("\tdescription \"{}\"\n", &tag.description).as_str();
			}

			if let Preview::Manual { sprite, animation } = tag.preview {
				source += format!("\tpreview \"{}\" \"{}\"\n", &sprite, &animation).as_str();
			}

			match tag.removescript {
				RemoveScript::Manual(removescript) => {
					// TODO: escape doublequotes
					source += format!("\tremovescript \"{}\"\n", &removescript).as_str();
				},
				RemoveScript::Auto => {
					source += "\tremovescript auto\n";
				},
				_ => ()
			}

			for script in tag.scripts {
				let Script::File { filename, supported_game } = script;
				source += format!("\tdescription \"{}\" {}\n", &filename, &supported_game).as_str();
			}

			for sprite in tag.sprites {
				source += format!("\tsprite \"{}\"\n", sprite.get_filename()).as_str();
				if let Sprite::Frames { frames, .. } = sprite {
					for frame in frames {
						source += format!("\t\tframe \"{}\"\n", frame.filename).as_str();
					}
				}
			}

			for background in tag.backgrounds {
				source += format!("\tbackground \"{}\"\n", &background.get_filename()).as_str();
			}

			for sound in tag.sounds {
				source += format!("\tsound \"{}\"\n", &sound.get_filename()).as_str();
			}

			for catalogue in tag.catalogues {
				source += format!("\tcatalogue \"{}\"\n", &catalogue.get_filename()).as_str();
			}
		}
	}

	Bytes::from(source)
}

fn split_c3ds_tags(tags: &Vec<Tag>) -> Vec<Tag> {
	let mut new_tags: Vec<Tag> = Vec::new();
	for tag in tags {
		match tag {
			Tag::Agent(tag) => {
				match tag.supported_game {
					SupportedGame::C3DS => {
						let mut c3_scripts: Vec<Script> = Vec::new();
						let mut ds_scripts: Vec<Script> = Vec::new();
						let mut c3_script_files: Vec<Bytes> = Vec::new();
						let mut ds_script_files: Vec<Bytes> = Vec::new();

						for (i, script) in tag.scripts.iter().enumerate() {
							let Script::File { supported_game, .. } = script;
							match supported_game {
								SupportedGame::C3 => {
									c3_scripts.push(script.clone());
									c3_script_files.push(tag.script_files.get(i).unwrap().clone());
								},
								SupportedGame::DS => {
									ds_scripts.push(script.clone());
									ds_script_files.push(tag.script_files.get(i).unwrap().clone());
								},
								SupportedGame::C3DS => {
									c3_scripts.push(script.clone());
									ds_scripts.push(script.clone());
									c3_script_files.push(tag.script_files.get(i).unwrap().clone());
									ds_script_files.push(tag.script_files.get(i).unwrap().clone());
								}
							}
						}

						println!("Split \"{}\" into \"{} C3\" and \"{} DS\"", tag.name, tag.name, tag.name);

						let mut c3_tag = tag.clone();
						c3_tag.name = format!("{} C3", tag.name);
						c3_tag.supported_game = SupportedGame::C3;
						c3_tag.scripts = c3_scripts;
						c3_tag.script_files = c3_script_files;
						new_tags.push(Tag::Agent(c3_tag));

						let mut ds_tag = tag.clone();
						ds_tag.name = format!("{} DS", tag.name);
						ds_tag.supported_game = SupportedGame::DS;
						ds_tag.scripts = ds_scripts;
						ds_tag.script_files = ds_script_files;
						new_tags.push(Tag::Agent(ds_tag));

					},
					_ => {
						new_tags.push(Tag::Agent(tag.clone()));
					}
				}
			}
			_ => {
				new_tags.push(tag.clone());
			}
		}
	}
	new_tags
}

pub fn compile(mut tags: Vec<Tag>) -> Bytes {
	for tag in &mut tags {
		tag.add_data();
	}
	println!();
	let tags = split_c3ds_tags(&tags);
	println!();
	pray::encode(&tags)
}

pub fn decompile(contents: &[u8], filename: &str) -> Result<Vec<FileData>, Box<dyn Error>> {
	let (tags, mut files) = pray::decode(contents)?;
	files.push(FileData {
		name: format!("{}.the", filename),
		data: encode_source(tags)
	});
	Ok(files)
}
