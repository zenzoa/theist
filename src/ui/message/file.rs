use crate::ui::{ Main, Selection };
use crate::ui::dialog::*;
use crate::pray::compile::compile as compile_pray;
use crate::pray::decompile::decompile as decompile_pray;
use crate::source::encode::encode as encode_source;
use crate::source::decode::decode as decode_source;
use crate::agent::tag::split_tags;
use crate::agent::file::only_used_files;
use crate::file_helper;

use std::str;
use std::io::*;
use std::fs;
use std::fs::File;
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
			save_file_as(main, main.filename.to_string());
		},

		FileMessage::Compile => {
			compile(main);
		}
	}
}

pub fn new_file(main: &mut Main) {
	if !main.modified || confirm_discard_changes() {
		main.filename = "untitled.the".to_string();
		main.path = "".to_string();
		main.tags = Vec::new();
		main.files = Vec::new();
		main.selection = Selection::None;
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
		if let Some(filepath) = file {
			open_file_from_path(main, filepath.to_string_lossy().into_owned());
		}
	}
}

pub fn open_file_from_path(main: &mut Main, filepath: String) {
	let path = file_helper::path(&filepath);
	let filename = file_helper::filename(&filepath);
	let extension = file_helper::extension(&filepath);

	match fs::read(&filepath) {
		Ok(contents) => {
			if extension == "agent" || extension == "agents" {
				match decompile_pray(&contents, false) {
					Ok(result) => {

						main.filename = filename;
						main.path = path;
						main.tags = result.tags;
						main.files = result.files;
						main.selected_tag_index = if main.tags.is_empty() { None } else { Some(0) };
						main.selection = Selection::None;
						main.modified = false;

					},
					Err(why) => {
						main.add_alert(&format!("ERROR: Unable to decompile agent file: {}", why), true);
					}
				}
			} else {
				match str::from_utf8(&contents) {
					Ok(contents) => {
						match decode_source(contents) {
							Ok(result) => {

								main.filename = filename;
								main.path = path;
								main.tags = result.tags;
								main.files = result.files;
								main.selected_tag_index = if main.tags.is_empty() { None } else { Some(0) };
								main.selection = Selection::None;
								main.modified = false;

								let mut files_with_missing_data: u32 = 0;
								for file in &mut main.files {
									match file.fetch_data(&main.path) {
										Ok(()) => (),
										Err(_why) => files_with_missing_data += 1
									}
								}
								if files_with_missing_data > 0 {
									main.add_alert(&format!("ERROR: Missing data for {} file(s)", files_with_missing_data), true);
								}

							},
							Err(why) => {
								main.add_alert(&format!("ERROR: Unable to understand theist file: {}", why), true);
							}
						}
					},
					Err(why) => {
						main.add_alert(&format!("ERROR: Unable to read theist file: {}", why), true);
					}
				}
			}
		},
		Err(why) => {
			main.add_alert(&format!("ERROR: Unable to open file: {}", why), true);
		}
	}
}

pub fn save_file(main: &mut Main) {
	let filepath = format!("{}{}", &main.path, &main.filename);
	let extension = file_helper::extension(&main.filename);
	if extension == "the" || extension == "txt" {
		if main.path.is_empty() || !file_helper::exists(&filepath) {
			save_file_as(main, main.filename.to_string());
		} else {
			save_file_to_path(main, filepath);
		}
	} else {
		save_file_as(main, format!("{}.the", file_helper::title(&main.filename)));
	}
}

pub fn save_file_as(main: &mut Main, mut default_filename: String) {
	let title = file_helper::title(&main.filename);
	let extension = file_helper::extension(&main.filename);
	if extension != "the" && extension != "txt" {
		default_filename = format!("{}.the", &title);
	}
	let file = FileDialog::new()
		.set_directory(&main.path)
		.set_file_name(&default_filename)
		.save_file();
	if let Some(filepath) = file {
		save_file_to_path(main, filepath.to_string_lossy().into_owned());
	}
}

pub fn save_file_to_path(main: &mut Main, filepath: String) {
	let extension = file_helper::extension(&main.filename);
	let is_agent = !(extension == "txt" || extension == "the");

	main.path = file_helper::path(&filepath);
	main.filename = file_helper::filename(&filepath);

	match encode_source(&main.tags, &mut main.files) {
		Ok(source_contents) => {
			match fs::write(filepath, source_contents) {
				Ok(()) => {
					if is_agent && !main.files.is_empty() && confirm_extract(main.files.len()) {
						extract_files(main);
					}
					main.modified = false;
				},
				Err(why) => {
					main.add_alert(&format!("ERROR: {}", why), true);
				}
			}
		},
		Err(why) => {
			main.add_alert(&format!("ERROR: {}", why), true);
		}
	}
}

pub fn extract_files(main: &mut Main) {
	let mut extracted_count: usize = 0;
	for file in &main.files {
		if let Some(data) = file.get_data() {
			let output_filename = format!("{}{}", &main.path, &file.get_output_filename());
			if let Ok(()) = fs::write(output_filename, &data) {
				extracted_count += 1;
			}
		}
	}
	main.add_alert(&format!("Extracted {} out of {} file(s)", extracted_count, main.files.len()), false);
}

pub fn compile(main: &mut Main) {
	for file in &mut main.files {
		file.fetch_data(&main.path).unwrap();
	}

	let (tags, mut files) = split_tags(&main.tags, &main.files);

	let mut missing_files = false;
	for file in &mut only_used_files(&tags, &files) {
		match file.get_data() {
			Some(data) => {
				if data.is_empty() {
					if let Err(_why) = file.fetch_data(&main.path) {
						missing_files = true;
					}
				}
			},
			None => {
				missing_files = true;
			}
		}
	}
	if missing_files {
		alert_missing_data();
		return;
	}

	let title = file_helper::title(&main.filename);
	let extension = if tags.len() > 1 { ".agents" } else { ".agent" };
	let filename = format!("{}{}", title, extension);

	let file = FileDialog::new()
		.set_directory(&main.path)
		.set_file_name(&filename)
		.save_file();

	if let Some(filepath) = file {
		let filepath = filepath.to_string_lossy().into_owned();

		match compile_pray(&tags, &mut files) {
			Ok(data) => {
				match File::create(filepath) {
					Ok(mut file) => {
						if let Err(why) = file.write_all(&data) {
							main.add_alert(&format!("ERROR: Unable to save agent file: {}", why), true);
						}
					},
					Err(why) => {
						main.add_alert(&format!("ERROR: Unable to save agent file: {}", why), true);
					}
				}
			},
			Err(why) => {
				main.add_alert(&format!("ERROR: Unable to compile agent file: {}", why), true);
			}
		}
	}
}
