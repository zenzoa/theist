use crate::agent::*;

use std::str;

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

fn add_agent_tag(path: &str, tokens: &Vec<String>) -> Tag {
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
	Tag::Agent(tag)
}

fn add_egg_tag(path: &str, tokens: &Vec<String>) -> Tag {
	let mut tag = EggTag::new(String::from(""));
	if let Some(i) = tokens.get(1) {
		tag.name = String::from(i);
	}
	tag.filepath = String::from(path);
	println!("Add egg \"{}\"", tag.name);
	println!("  Path: {}", tag.filepath);
	Tag::Egg(tag)
}

pub fn decode_source(contents: &str, path: &str) -> Vec<Tag> {
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
					"agent" => tags.push(add_agent_tag(path, &tokens)),
					"egg" => tags.push(add_egg_tag(path, &tokens)),
					_ => ()
				}
			},
			_ => {
				match token {
					"agent" => tags.push(add_agent_tag(path, &tokens)),
					"egg" => tags.push(add_egg_tag(path, &tokens)),
					_ => ()
				}
			}
		}
	}

	tags
}
