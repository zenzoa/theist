use crate::pray;
use crate::agent;
use crate::agent::tag::*;
use crate::agent::agent_tag::*;
use crate::agent::script::*;
use crate::agent::sprite::*;
use crate::agent::background::*;
use crate::agent::sound::*;
use crate::agent::catalogue::*;
use crate::agent::genetics::*;
use crate::agent::body_data::*;
use crate::agent::encode::*;
use crate::agent::decode::*;
use crate::ui::{ Main, SelectionType };
use crate::ui::dialogs::*;

use std::str;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::io::prelude::*;
use rfd::FileDialog;

#[derive(Debug, Clone)]
pub enum FileMessage {
	New,
	Open,
	Save,
	SaveAs,
	Compile
}

pub fn check_file_message(main: &mut Main, message: FileMessage) {
	match message {
		FileMessage::New => {
			new_file(main);
		},

		FileMessage::Open => {
			open_file(main);
		},

		FileMessage::Save => {
			save_file(main);
		},

		FileMessage::SaveAs => {
			save_file_as(main, main.filename.clone());
		},

		FileMessage::Compile => {
			compile(main);
		}
	}
}

pub fn new_file(main: &mut Main) {
	if !main.modified || confirm_discard_changes() {
		main.filename = String::from("untitled.the");
		main.path = String::from("");
		main.tags = vec![ Tag::Agent(AgentTag::new(String::from("My Agent"))) ];
		main.selected_tag = Some(0);
		main.selection_type = SelectionType::Tag;
		main.alerts = Vec::new();
		main.modified = false;
	}
}

pub fn open_file(main: &mut Main) {
	if !main.modified || confirm_discard_changes() {
		let file = FileDialog::new()
			.add_filter("theist or agent", &["the", "txt", "agent", "agents"])
			.set_directory(&main.path)
			.pick_file();
		if let Some(path) = file {
			open_file_from_path(main, path);
		}
	}
}

pub fn open_file_from_path(main: &mut Main, path: PathBuf) {
	main.clear_alerts();

	let filepath = match path.parent() {
		Some(parent) => parent.to_string_lossy().into_owned() + "/",
		None => String::from("")
	};
	let filename = match path.file_name() {
		Some(filename) => filename.to_string_lossy().into_owned(),
		None => String::from("untitled.the")
	};
	let extension = match path.extension() {
		Some(extension) => extension.to_string_lossy().into_owned(),
		None => String::from("")
	};

	match fs::read(format!("{}{}", &filepath, &filename)) {
		Ok(contents) => {
			if extension == "agent" || extension == "agents" {
				match pray::decode(&contents) {
					Ok((tags, files)) => {
						main.set_path_and_name(&path);
						main.tags = tags;
						if main.tags.is_empty() {
							main.selected_tag = None;
						} else {
							main.selected_tag = Some(0);
						}
						main.files = files;
						main.modified = false;
					},
					Err(why) => main.add_alert(format!("ERROR: {}", why).as_str(), true)
				}
			} else {
				match str::from_utf8(&contents) {
					Ok(contents) => {
						main.set_path_and_name(&path);
						main.tags = decode_source(contents, &main.path);
						if main.tags.is_empty() {
							main.selected_tag = None;
							main.add_alert("No tags found in file", true);
						} else {
							main.selected_tag = Some(0);
						}
						main.modified = false;
					},
					Err(why) => main.add_alert(format!("ERROR: {}", why).as_str(), true)
				}
			}
		},
		Err(why) => main.add_alert(format!("ERROR: Unable to open file: {}", why).as_str(), true)
	}
}

pub fn save_file(main: &mut Main) {
	let mut filename = main.filename.clone();
	let is_pray_file = filename.ends_with(".agent") || filename.ends_with(".agents");
	if filename.ends_with(".agent") {
		filename = filename.replace(".agent", ".the");
	} else if filename.ends_with(".agents") {
		filename = filename.replace(".agents", ".the");
	}

	let filepath = format!("{}{}", &main.path, filename);
	let file_exists = match File::open(&filepath) {
		Ok(_file) => true,
		Err(_why) => false
	};

	if file_exists && !is_pray_file {
		save_file_to_path(main, PathBuf::from(filepath));
		main.modified = false;
	} else {
		save_file_as(main, filename);
	}
}

pub fn save_file_as(main: &mut Main, default_filename: String) {
	let file = FileDialog::new()
		.set_directory(&main.path)
		.set_file_name(&default_filename)
		.save_file();
	if let Some(path) = file {
		save_file_to_path(main, path);
		main.modified = false;
	}
}

pub fn save_file_to_path(main: &mut Main, path: PathBuf) {
	main.clear_alerts();

	main.set_path_and_name(&path);
	let data = encode_source(main.tags.clone());
	let filepath = format!("{}{}", main.path, main.filename);
	match File::create(filepath) {
		Ok(mut file) => {
			file.write_all(&data).unwrap();
			if !main.files.is_empty() && confirm_extract(main.files.len()) {
				extract_files(main);
			}
		},
		Err(why) => {
			main.add_alert(format!("ERROR: {}", why).as_str(), true);
		}
	}
}

pub fn extract_files(main: &mut Main) {
	let mut alerts: Vec<String> = Vec::new();
	for inline_file in &main.files {
		match File::create(format!("{}{}", &main.path, &inline_file.name)) {
			Ok(mut file) => {
				file.write_all(&inline_file.data).unwrap();
			},
			Err(why) => {
				alerts.push(format!("ERROR: Unable to extract file {}: {}", &inline_file.name, why));
			}
		}
	}
	for alert in alerts {
		main.add_alert(alert.as_str(), true);
	}
}

pub fn compile(main: &mut Main) {
	main.clear_alerts();
	let extension = if main.tags.len() > 1 { ".agents" } else { ".agent" };
	let filename = main.filename.replace(".the", extension).replace(".txt", extension);
	let file = FileDialog::new()
		.set_directory(&main.path)
		.set_file_name(&filename)
		.save_file();
	if let Some(path) = file {
		let data = agent::compile(main.tags.clone());
		if let Ok(mut file) = File::create(path) {
			file.write_all(&data).unwrap();
			main.add_alert(format!("SUCCESS! File compiled successfully: {}", &filename).as_str(), false);
		}
	}
}

pub fn add_file(main: &mut Main) {
	let file = FileDialog::new()
		.add_filter("Creatures Files", &["cos", "c16", "blk", "wav", "catalogue", "png", "gen", "gno", "att"])
		.set_directory(&main.path)
		.pick_file();
	if let Some(file_path) = file {
		add_file_from_path(main, file_path, false);
	}
}

pub fn add_file_from_path(main: &mut Main, file_path: PathBuf, file_dropped: bool) {
	main.clear_alerts();

	let path = match file_path.parent() {
		Some(parent) => parent.to_string_lossy().into_owned() + "/",
		None => String::from("")
	};
	let filename = match file_path.file_name() {
		Some(filename) => filename.to_string_lossy().into_owned(),
		None => String::from("")
	};
	let title = match file_path.file_stem() {
		Some(file_stem) => file_stem.to_string_lossy().into_owned(),
		None => String::from("")
	};
	let extension = match file_path.extension() {
		Some(extension) => extension.to_string_lossy().into_owned(),
		None => String::from("")
	};

	if file_dropped
		&& (extension == "the" || extension == "txt" || extension == "agent" || extension == "agents")
		&& (!main.modified || confirm_discard_changes()) {
			open_file_from_path(main, file_path);
			return;
	}

	if main.path.is_empty() {
		main.path = path;
	} else if main.path != path {
		alert_wrong_folder();
		return;
	}

	if let Some(selected_tag) = main.selected_tag {
		match &mut main.tags[selected_tag] {
			Tag::Agent(tag) => {
				if tag.filepath.is_empty() {
					tag.filepath = main.path.clone();
				}
				match extension.as_str() {
					"cos" => {
						if !tag.scripts.includes(&filename) {
							tag.scripts.push(Script::new(&filename, &tag.supported_game.to_string()));
							main.selection_type = SelectionType::Script(tag.scripts.len() - 1);
							main.modified = true;
						}
					},
					"c16" => {
						if !tag.sprites.includes(&filename) {
							tag.sprites.push(Sprite::new(&filename));
							main.selection_type = SelectionType::Sprite(tag.sprites.len() - 1);
							main.modified = true;
						}
					},
					"blk" => {
						if !tag.backgrounds.includes(&filename) {
							tag.backgrounds.push(Background::new(&filename));
							main.selection_type = SelectionType::Background(tag.backgrounds.len() - 1);
							main.modified = true;
						}
					},
					"wav" => {
						if !tag.sounds.includes(&filename) {
							tag.sounds.push(Sound::new(&filename));
							main.selection_type = SelectionType::Sound(tag.sounds.len() - 1);
							main.modified = true;
						}
					},
					"catalogue" => {
						if !tag.catalogues.includes(&filename) {
							tag.catalogues.push(Catalogue::new(&filename));
							main.selection_type = SelectionType::Catalogue(tag.catalogues.len() - 1);
							main.modified = true;
						}
					},
					"png" => {
						if !tag.sprites.includes(&filename) {
							let mut sprite = Sprite::new(format!("{}.c16", &title).as_str());
							sprite.add_frame(&filename);
							tag.sprites.push(sprite);
							main.selection_type = SelectionType::Sprite(tag.sprites.len() - 1);
							main.modified = true;
						}
					},
					_ => {
						alert_wrong_filetype("COS, C16, BLK, WAV, CATALOGUE, or PNG");
					}
				}
				main.modified = true;
			},

			Tag::Egg(tag) => {
				if tag.filepath.is_empty() {
					tag.filepath = main.path.clone();
				}
				match extension.as_str() {
					"c16" => {
						if !tag.sprites.includes(&filename) {
							tag.sprites.push(Sprite::new(&filename));
							main.selection_type = SelectionType::Sprite(tag.sprites.len() - 1);
							main.modified = true;
						}
					},
					"png" => {
						if !tag.sprites.includes(&filename) {
							let mut sprite = Sprite::new(format!("{}.c16", &title).as_str());
							sprite.add_frame(&filename);
							tag.sprites.push(sprite);
							main.selection_type = SelectionType::Sprite(tag.sprites.len() - 1);
							main.modified = true;
						}
					},
					"gen" => {
						if !tag.genetics.includes(&filename) {
							if tag.genetics.len() >= 2 {
								alert_too_many_genetics_files();
							} else {
								let new_genetics = Genetics::new(&filename);
								if let Some(genetics) = tag.genetics.get(0) {
									if genetics.filename.title != new_genetics.filename.title {
										alert_wrong_genetics_title();
										return;
									}
								}
								tag.genetics.push(new_genetics);
								main.selection_type = SelectionType::Genetics(tag.genetics.len() - 1);
								main.modified = true;
							}
						}
					},
					"gno" => {
						if !tag.genetics.includes(&filename) {
							if tag.genetics.len() >= 2 {
								alert_too_many_genetics_files();
							} else {
								let new_genetics = Genetics::new(&filename);
								if let Some(genetics) = tag.genetics.get(0) {
									if genetics.filename.title != new_genetics.filename.title {
										alert_wrong_genetics_title();
										return;
									}
								}
								tag.genetics.push(new_genetics);
								main.selection_type = SelectionType::Genetics(tag.genetics.len() - 1);
								main.modified = true;
							}
						}
					},
					"att" => {
						if !tag.body_data.includes(&filename) {
							tag.body_data.push(BodyData::new(&filename));
							main.selection_type = SelectionType::BodyData(tag.body_data.len() - 1);
							main.modified = true;
						}
					},

					_ => {
						alert_wrong_filetype("C16, PNG, GEN, GNO, or ATT");
					}
				}
			},
			Tag::Empty => ()
		}
	}
}
