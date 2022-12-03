use std::fmt;
use std::env;
use std::fs;
use regex::Regex;

#[derive(Clone)]
struct Filename {
	title: String,
	extension: String
}

impl Filename {
	fn new(filename_string: &str, fallback_extension: &str) -> Filename {
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
}

impl fmt::Display for Filename {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	write!(f, "{}.{}", &self.title, &self.extension)
    }
}

struct Filepath(String);

enum SupportedGame {
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

enum Script {
	File { filename: Filename, supported_game: SupportedGame },
	//Inline { contents: String, supported_game: SupportedGame }
}

impl Script {
	fn new(filename: &str, supported_game: &str) -> Script {
		Script::File {
			filename: Filename::new(filename, "cos"),
			supported_game: SupportedGame::new(supported_game)
		}
	}
}

struct SpriteFrame {
	filename: Filename
}

impl SpriteFrame {
	fn new(filename: &str) -> SpriteFrame {
		SpriteFrame {
			filename: Filename::new(filename, "png")
		}
	}
}

enum Sprite {
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
}

enum Background {
	BLK { filename: Filename },
	PNG { filename: Filename }
}

impl Background {
	fn new(filename: &str) -> Background {
		let filename = Filename::new(filename, "png");
		match filename.extension.as_str() {
			"blk" => Background::BLK { filename },
			_ => Background::PNG { filename }
		}
	}
}


struct Sound {
	filename: Filename
}

impl Sound {
	fn new(filename: &str) -> Sound {
		Sound {
			filename: Filename::new(filename, "wav")
		}
	}
}

struct CatalogueEntry {
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

enum Catalogue {
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
}

//struct Genetics {
//	filename: Filename
//}

enum RemoveScript {
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

enum InjectorPreview {
	Auto,
	Manual { sprite: String, animation: String }
}

struct AgentTag {
	filepath: Filepath,
	name: String,
	version: String,
	description: String,
	supported_game: SupportedGame,
	remove_script: RemoveScript,
	injector_preview: InjectorPreview,
	scripts: Vec<Script>,
	sprites: Vec<Sprite>,
	backgrounds: Vec<Background>,
	sounds: Vec<Sound>,
	catalogues: Vec<Catalogue>
}

impl AgentTag {
	fn new() -> AgentTag {
		AgentTag {
			filepath: Filepath(String::from("")),
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
		}
	}
}

//struct EggTag {
//	filepath: Filepath,
//	name: String,
//	version: String,
//	preview_sprite_male: String,
//	preview_sprite_female: String,
//	preview_animation: String,
//	genetics: Vec<Genetics>,
//	sprites: Vec<Sprite>
//}

enum Tag {
	Empty,
	Agent(AgentTag),
	//Egg(EggTag)
}

fn parse_tokens(s:&str) -> Vec<String> {
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
					} else if current_token.len() > 0 {
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
	if current_token.len() > 0 {
		tokens.push(current_token.clone());
	}
	return tokens;
}

fn parse_source(contents: &str) {
	//let contents = "agent \"Aibo Ball\" c3ds\n\nversion \"1.0.4\"\ndescription \"We took the iconic pink ball away from an aibo so your norns can play with it instead.\"\npreview \"aibo_ball.c16\" \"1 2 3 3\"\nremovescript \"enum 000\"\nscript \"aibo_ball.cos\" ds\nsprite \"aibo_ball.c16\"\nframe \"aibo_ball1.png\"\nframe \"aibo_ball2.png\"\nframe \"aibo_ball3.png\"\nbackground \"bg.png\"\nsound \"blop.wav\"\ncatalogue \"aibo_ball.catalogue\"\nentry \"2 21 21212\" \"Aibo Ball\" \"We took the iconic pink ball away from an aibo so your norns can play with it instead.\"";
	
	let mut tags: Vec<Tag> = Vec::new();
	for line in contents.lines() {
		let tokens = parse_tokens(line.trim());
		if tokens.len() == 0 {
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
						println!("set version: {}", tag.version);
					},
					"description" => {
						if let Some(i) = tokens.get(1) {
							tag.description = String::from(i);
						}
						println!("set description: {}", tag.description);
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
						if sprite.len() > 0 {
							println!("set preview: {} \"{}\"", sprite, animation);
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
						println!("set remove script: {}", tag.remove_script)
					},
					"script" => {
						if let Some(filename) = tokens.get(1) {
							let supported_game = match tokens.get(2) {
								None => "c3ds",
								Some(i) => i.as_str()
							};
							let script = Script::new(filename, supported_game);
							tag.scripts.push(script);
							println!("add script! script count: {}", tag.scripts.len());
						}
					},
					"sprite" => {
						if let Some(filename) = tokens.get(1) {
							let sprite = Sprite::new(filename);
							tag.sprites.push(sprite);
							println!("add sprite! sprite count: {}", tag.scripts.len());
						}
					},
					"frame" => {
						let num_sprites = tag.sprites.len();
						if let Some(current_sprite) = tag.sprites.get_mut(num_sprites - 1) {
							if let Some(filename) = tokens.get(1) {
								let frame = SpriteFrame::new(filename);
								current_sprite.add_frame(frame);
								if let Sprite::Frames { frames, .. } = current_sprite {
									println!("> add frame {}", frames.len());
								}
							}
						}
					},
					"background" => {
						if let Some(filename) = tokens.get(1) {
							let background = Background::new(filename);
							tag.backgrounds.push(background);
							println!("add background! background count: {}", tag.backgrounds.len());
						}
					},
					"sound" => {
						if let Some(filename) = tokens.get(1) {
							let sound = Sound::new(filename);
							tag.sounds.push(sound);
							println!("add sound! sound count: {}", tag.sounds.len());
						}
					},
					"catalogue" => {
						if let Some(filename) = tokens.get(1) {
							let catalogue = Catalogue::new(filename);
							tag.catalogues.push(catalogue);
							println!("add catalogue! catalogue count: {}", tag.catalogues.len());
						}
					},
					"entry" => {
						let num_catalogues = tag.catalogues.len();
						if let Some(current_catalogue) = tag.catalogues.get_mut(num_catalogues - 1) {
							if let (Some(classifier), Some(name), Some(description)) = (tokens.get(1), tokens.get(2), tokens.get(3)) {
								let entry = CatalogueEntry::new(classifier, name, description);
								current_catalogue.add_entry(entry);
								if let Catalogue::Inline { entries, .. } = current_catalogue {
									println!("> add entry {}", entries.len());
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
						println!("agent name: {}", tag.name);
						println!("supported game: {}", tag.supported_game);
						tags.push(Tag::Agent(tag));
					},
					"egg" => {
						println!("create egg tag");
					},
					_ => ()
				}
			}
		}
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	if let Some(file_path) = &args.get(1) {
		println!("OPEN FILE: {}", file_path);
		if let Result::Ok(contents) = fs::read_to_string(file_path) {
			parse_source(&contents);
		}
	}
}
