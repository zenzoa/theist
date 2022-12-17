use crate::c16;
use crate::blk;
use crate::pray;

use std::fmt;
use std::fs;
use std::io;
use std::str;
use std::error::Error;
use regex::Regex;
use image::RgbaImage;
use image::io::Reader as ImageReader;
use bytes::Bytes;

pub struct FileData {
	pub name: String,
	pub data: Bytes
}

#[derive(Clone)]
pub struct Filename {
	pub title: String,
	pub extension: String
}

impl Filename {
	pub fn new(filename_string: &str, fallback_extension: &str) -> Filename {
		let filename_pattern = Regex::new(r"^(.+)\.(.+)$").unwrap();
		match filename_pattern.captures(filename_string) {
			None => Filename {
				title: String::from(""), extension: String::from(fallback_extension)
			},
			Some(captures) => Filename {
				title: String::from(&captures[1]),
				extension: String::from(&captures[2])
			}
		}
	}

	pub fn as_string(&self) -> String {
		format!("{}.{}", &self.title, &self.extension)
	}
}

impl fmt::Display for Filename {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}.{}", &self.title, &self.extension)
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
	fn new(s:&str) -> SupportedGame {
		match s {
			"c3" => SupportedGame::C3,
			"C3" => SupportedGame::C3,
			"ds" => SupportedGame::DS,
			"DS" => SupportedGame::DS,
			_ => SupportedGame::C3DS
		}
	}
}

#[derive(Clone)]
pub enum Script {
	File { filename: Filename, supported_game: SupportedGame },
	//Inline { contents: String, supported_game: SupportedGame }
}

impl Script {
	pub fn new(filename: &str, supported_game: &str) -> Script {
		Script::File {
			filename: Filename::new(filename, "cos"),
			supported_game: SupportedGame::new(supported_game)
		}
	}

	fn get_data(&self, path: &str) -> Result<Bytes, io::Error> {
		match self {
			Script::File { filename, .. } => {
				let filepath = format!("{}{}", path, filename);
				let contents = fs::read(&filepath)?;
				println!("  Got data from {}", &filepath);
				Ok(Bytes::copy_from_slice(&contents))
			}
		}
	}
}

#[derive(Clone)]
pub struct SpriteFrame {
	pub filename: Filename
}

impl SpriteFrame {
	fn new(filename: &str) -> SpriteFrame {
		SpriteFrame {
			filename: Filename::new(filename, "png")
		}
	}
}

#[derive(Clone)]
pub enum Sprite {
	C16 { filename: Filename },
	Frames { filename: Filename, frames: Vec<SpriteFrame> },
	//Spritesheet { filename: Filename, spritesheet_filename: Filename, cols: u32, rows: u32 }
}

impl Sprite {
	fn new(filename: &str) -> Sprite {
		Sprite::C16 {
			filename: Filename::new(filename, "cos")
		}
	}

	fn add_frame(&mut self, frame: SpriteFrame) {
		match self {
			Sprite::C16 { filename } => {
				*self = Sprite::Frames {
					filename: filename.clone(),
					frames: vec![ frame ]
				}
			},
			Sprite::Frames { frames, .. } => {
				frames.push(frame);
			}
		}
	}

	pub fn get_filename(&self) -> String {
		match self {
			Sprite::C16 { filename } => filename.as_string(),
			Sprite::Frames { filename, .. } => filename.as_string()
		}
	}

	pub fn get_title(&self) -> String {
		match self {
			Sprite::C16 { filename } => filename.title.clone(),
			Sprite::Frames { filename, .. } => filename.title.clone()
		}
	}

	fn get_data(&self, path: &str) -> Result<Bytes, Box<dyn Error>> {
		match self {
			Sprite::C16 { filename, .. } => {
				let filepath = format!("{}{}", path, filename);
				let contents = fs::read(&filepath)?;
				println!("  Got data from {}", &filepath);
				Ok(Bytes::copy_from_slice(&contents))
			},
			Sprite::Frames { frames, .. } => {
				let mut images: Vec<RgbaImage> = Vec::new();
				for frame in frames {
					let filepath = format!("{}{}", path, frame.filename);
					let image_data = ImageReader::open(&filepath)?;
					let image = image_data.decode()?;
					println!("  Got data from {}", &filepath);
					images.push(image.into_rgba8());
				}
				Ok(Bytes::from(c16::encode(images)))
			}
		}
	}
}

#[derive(Clone)]
pub enum Background {
	Blk { filename: Filename },
	Png { filename: Filename }
}

impl Background {
	fn new(filename: &str) -> Background {
		let filename = Filename::new(filename, "png");
		match filename.extension.as_str() {
			"blk" => Background::Blk { filename },
			_ => Background::Png { filename }
		}
	}

	pub fn get_filename(&self) -> String {
		match self {
			Background::Blk { filename } => filename.as_string(),
			Background::Png { filename } => filename.as_string()
		}
	}

	pub fn get_title(&self) -> String {
		match self {
			Background::Blk { filename } => filename.title.clone(),
			Background::Png { filename } => filename.title.clone()
		}
	}

	fn get_data(&self, path: &str) -> Result<Bytes, Box<dyn Error>> {
		match self {
			Background::Blk { filename } => {
				let filepath = format!("{}{}", path, filename);
				let contents = fs::read(&filepath)?;
				println!("  Got data from {}", &filepath);
				Ok(Bytes::copy_from_slice(&contents))
			},
			Background::Png { filename } => {
				let filepath = format!("{}{}", path, filename);
				let image_data = ImageReader::open(&filepath)?;
				let image = image_data.decode()?;
				println!("  Got data from {}", &filepath);
				Ok(Bytes::from(blk::encode(image.into_rgba8())))
			}
		}
	}
}

#[derive(Clone)]
pub struct Sound {
	pub filename: Filename
}

impl Sound {
	fn new(filename: &str) -> Sound {
		Sound {
			filename: Filename::new(filename, "wav")
		}
	}

	pub fn get_filename(&self) -> String {
		self.filename.as_string()
	}

	fn get_data(&self, path: &str) -> Result<Bytes, Box<dyn Error>> {
		let filepath = format!("{}{}", path, self.filename);
		let contents = fs::read(&filepath)?;
		println!("  Got data from {}", &filepath);
		Ok(Bytes::copy_from_slice(&contents))
	}
}

#[derive(Clone)]
pub struct CatalogueEntry {
	classifier: String,
	name: String,
	description: String
}

impl CatalogueEntry {
	fn new(classifier: &str, name: &str, description: &str) -> CatalogueEntry {
		CatalogueEntry {
			classifier: String::from(classifier),
			name: String::from(name),
			description: String::from(description)
		}
	}
}

#[derive(Clone)]
pub enum Catalogue {
	File { filename: Filename },
	Inline { filename: Filename, entries: Vec<CatalogueEntry> }
}

impl Catalogue {
	fn new(filename: &str) -> Catalogue {
		Catalogue::File {
			filename: Filename::new(filename, "catalogue")
		}
	}

	fn add_entry(&mut self, entry: CatalogueEntry) {
		match self {
			Catalogue::File { filename } => {
				*self = Catalogue::Inline {
					filename: filename.clone(),
					entries: vec![ entry ]
				}
			},
			Catalogue::Inline { entries, .. } => {
				entries.push(entry);
			}
		}
	}

	pub fn get_filename(&self) -> String {
		match self {
			Catalogue::File { filename } => filename.as_string(),
			Catalogue::Inline { filename, .. } => filename.as_string()
		}
	}

	fn get_data(&self, path: &str) -> Result<Bytes, Box<dyn Error>> {
		match self {
			Catalogue::File { filename } => {
				let filepath = format!("{}{}", path, filename);
				let contents = fs::read(&filepath)?;
				println!("  Got data from {}", &filepath);
				Ok(Bytes::copy_from_slice(&contents))
			},
			Catalogue::Inline { filename, entries } => {
				let mut contents = String::new();
				for entry in entries {
					contents += format!(
						"TAG \"Agent Help {}\"\n\"{}\"\n\"{}\"\n\n",
						entry.classifier,
						entry.name,
						entry.description
					).as_str();
				}
				println!("  Catalogue created: {}", filename);
				Ok(Bytes::copy_from_slice(contents.as_bytes()))
			}
		}
	}
}

//struct Genetics {
//	filename: Filename
//}

#[derive(Clone)]
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

#[derive(Clone)]
pub enum InjectorPreview {
	Auto,
	Manual { sprite: String, animation: String }
}

#[derive(Clone)]
pub struct AgentTag {
	pub filepath: String,
	pub name: String,
	pub version: String,
	pub description: String,
	pub supported_game: SupportedGame,
	pub remove_script: RemoveScript,
	pub injector_preview: InjectorPreview,

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
			remove_script: RemoveScript::None,
			injector_preview: InjectorPreview::Auto,

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

//struct EggTag {
//	filepath: String,
//	name: String,
//	version: String,
//	preview_sprite_male: String,
//	preview_sprite_female: String,
//	preview_animation: String,
//	genetics: Vec<Genetics>,
//	sprites: Vec<Sprite>
//}

#[derive(Clone)]
pub enum Tag {
	Empty,
	Agent(AgentTag),
	//Egg(EggTag)
}

impl Tag {
	fn add_data(&mut self) {
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
					if let RemoveScript::Auto = tag.remove_script {
						match str::from_utf8(&tag.script_files[0]) {
							Ok(script) => {
								let remove_script_pattern = Regex::new(r"[\n\r]rscr[\n\r]([\s\S]*)").unwrap();
								match remove_script_pattern.captures(script) {
									Some(captures) => {
										match captures.get(1) {
											Some(remove_script) => {
												let remove_newlines_pattern = Regex::new(r"\s+").unwrap();
												let remove_script = String::from(
													remove_newlines_pattern.replace_all(remove_script.as_str(), " ").trim()
												);
												println!("  Remove script extracted from first script");
												tag.remove_script = RemoveScript::Manual(remove_script);
											},
											None => {
												println!("ERROR: No remove script found in first script.");
												tag.remove_script = RemoveScript::None;
											}
										}

									},
									None => {
										println!("ERROR: No remove script found in first script.");
										tag.remove_script = RemoveScript::None;
									}
								}
							},
							Err(why) => {
								println!("ERROR: Unable to extract remove script from first script: {}", why);
								tag.remove_script = RemoveScript::None;
							}
						}
					}
				}

				// injector preview
				if !tag.sprites.is_empty() {
					if let InjectorPreview::Auto = tag.injector_preview {
						let sprite_name = &tag.sprites[0].get_title();
						tag.injector_preview = InjectorPreview::Manual {
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
							tag.injector_preview = InjectorPreview::Manual{ sprite, animation };
						}
					},
					"removescript" => {
						tag.remove_script = match tokens.get(1) {
							None => RemoveScript::None,
							Some(i) => match i.as_str() {
								"auto" => RemoveScript::Auto,
								_ => RemoveScript::Manual(i.to_string())
							}
						};
						println!("  Remove script: {}", tag.remove_script)
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
								let frame = SpriteFrame::new(filename);
								current_sprite.add_frame(frame);
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
						let mut tag = AgentTag::new();
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

			if let InjectorPreview::Manual { sprite, animation } = tag.injector_preview {
				source += format!("\tpreview \"{}\" \"{}\"\n", &sprite, &animation).as_str();
			}

			match tag.remove_script {
				RemoveScript::Manual(remove_script) => {
					source += format!("\tremovescript \"{}\"\n", &remove_script).as_str();
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
