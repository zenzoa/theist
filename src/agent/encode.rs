use crate::agent::*;
use crate::agent::script::ScriptList;

use bytes::Bytes;

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

			for script in tag.scripts.iter() {
				let Script::File { filename, supported_game } = script;
				source += format!("\tdescription \"{}\" {}\n", &filename, &supported_game).as_str();
			}

			for sprite in tag.sprites.iter() {
				source += format!("\tsprite \"{}\"\n", sprite.get_filename()).as_str();
				if let Sprite::Frames { frames, .. } = sprite {
					for frame in frames {
						source += format!("\t\tframe \"{}\"\n", frame.filename).as_str();
					}
				}
			}

			for background in tag.backgrounds.iter() {
				source += format!("\tbackground \"{}\"\n", &background.get_filename()).as_str();
			}

			for sound in tag.sounds.iter() {
				source += format!("\tsound \"{}\"\n", &sound.get_filename()).as_str();
			}

			for catalogue in tag.catalogues.iter() {
				source += format!("\tcatalogue \"{}\"\n", &catalogue.get_filename()).as_str();
			}
		}
	}

	Bytes::from(source)
}

pub fn split_tags(tags: &Vec<Tag>) -> Vec<Tag> {
	let mut new_tags: Vec<Tag> = Vec::new();
	for tag in tags {
		match tag {
			Tag::Agent(tag) => {
				match tag.supported_game {
					SupportedGame::C3DS => {
						let mut c3_scripts = ScriptList::new();
						let mut ds_scripts = ScriptList::new();
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
