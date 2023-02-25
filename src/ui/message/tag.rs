use crate::ui::{ Main, Selection };
use crate::ui::dialog::*;
use crate::agent::tag::Tag;
use crate::agent::agent_tag::{ AgentTag, SupportedGame, Preview, RemoveScript };
use crate::agent::egg_tag::{ EggTag, EggPreview };
use crate::agent::free_tag::FreeTag;
use crate::agent::script::Script;
use crate::agent::sprite::{ Sprite, SpriteFrame };
use crate::agent::sound::Sound;
use crate::agent::catalogue::{ Catalogue, CatalogueEntry };
use crate::agent::bodydata::BodyData;
use crate::agent::genetics::Genetics;
use crate::agent::file::{ CreaturesFile, FileType, lookup_file_index };
use crate::error::create_error;
use crate::file_helper;

use std::fs;
use rfd::FileDialog;
use std::error::Error;
use bytes::Bytes;

#[derive(Debug, Clone)]
pub enum TagMessage {
	Select(usize),
	AddAgentTag,
	AddEggTag,
	Remove,
	SetName(String),
	SetDescription(String),
	SetVersion(String),
	SetSupportedGame(usize),

	SetPreviewType(usize),
	SetPreviewSprite(String),
	SetPreviewAnimation(String),

	SetEggPreviewType(bool),
	SetEggPreviewSpriteMale(String),
	SetEggPreviewSpriteFemale(String),
	SetEggPreviewAnimation(String),

	SetRemoveScriptType(usize),
	SetRemoveScript(String),

	SetGenome(String),

	SelectFile(FileType, usize),
	MoveFileUp(FileType, usize),
	MoveFileDown(FileType, usize),
	RemoveFile(FileType, usize, String),
	AddFile,
	AddExistingFile(usize),
	AddInlineCatalogue
}

pub fn check_tag_message(main: &mut Main, message: TagMessage) {
	match message {
		TagMessage::Select(index) => {
			if index < main.tags.len() {
				main.selected_tag_index = Some(index);
				main.selection = Selection::None;
			}
		},

		TagMessage::AddAgentTag => {
			let mut new_tag = AgentTag::new();
			if main.tags.is_empty() {
				new_tag.use_all_files = true; // default to using all files for first tag
			}
			main.tags.push(Tag::Agent(new_tag));
			finish_adding_tag(main);
		},

		TagMessage::AddEggTag => {
			let mut new_tag = EggTag::new();
			if main.tags.is_empty() {
				new_tag.use_all_files = true; // default to using all files for first tag
			}
			main.tags.push(Tag::Egg(new_tag));
			finish_adding_tag(main);
		},

		TagMessage::Remove => {
			if confirm_remove_tag() {
				if let Some(index) = main.selected_tag_index {
					main.tags.remove(index);
					if main.tags.is_empty() {
						main.selected_tag_index = None;
					} else if index > 0 {
						main.selected_tag_index = Some(index - 1);
					}
					main.selection = Selection::None;
					main.modified = true;
				}
			}
		},

		TagMessage::SetName(new_name) => {
			if let Some(tag) = main.get_selected_tag_mut() {
				match tag {
					Tag::Agent(agent_tag) => { agent_tag.name = new_name; },
					Tag::Egg(egg_tag) => { egg_tag.name = new_name; },
					Tag::Free(free_tag) => { free_tag.name = new_name; }
				}
				main.modified = true;
			}
		},

		TagMessage::SetDescription(new_description) => {
			if let Some(Tag::Agent(agent_tag)) = main.get_selected_tag_mut() {
				agent_tag.description = new_description;
				main.modified = true;
			}
		},

		TagMessage::SetVersion(new_version) => {
			if let Some(tag) = main.get_selected_tag_mut() {
				match tag {
					Tag::Agent(agent_tag) => { agent_tag.version = new_version; },
					Tag::Egg(egg_tag) => { egg_tag.version = new_version; },
					Tag::Free(free_tag) => { free_tag.version = new_version; }
				}
				main.modified = true;
			}
		},

		TagMessage::SetSupportedGame(new_supported_game) => {
			if let Some(Tag::Agent(agent_tag)) = main.get_selected_tag_mut() {
				match new_supported_game {
					0 => agent_tag.supported_game = SupportedGame::C3DS,
					1 => agent_tag.supported_game = SupportedGame::C3,
					2 => agent_tag.supported_game = SupportedGame::DS,
					_ => ()
				}
				main.modified = true;
			}
		},

		TagMessage::SetPreviewType(new_type) => {
			let files = main.files.clone();
			if let Some(Tag::Agent(agent_tag)) = main.get_selected_tag_mut() {
				if let Preview::Manual{ sprite: _, animation: _ } = &agent_tag.preview {
					agent_tag.preview_backup = agent_tag.preview.clone();
				}

				match new_type {
					0 => {
						agent_tag.preview = Preview::None;
						main.modified = true;
					},

					1 => {
						agent_tag.preview = Preview::Auto;
						main.modified = true;
					},

					_ => {
						if let Some(sprite_index) = agent_tag.get_first_sprite(&files) {
							let mut new_sprite = sprite_index.clone();
							let mut new_animation = "0".to_string();

							if let Preview::Manual{ sprite, animation } = &agent_tag.preview_backup {
								if agent_tag.has_sprite(*sprite) { new_sprite = *sprite; }
								new_animation = animation.to_string();
							}

							agent_tag.preview = Preview::Manual{
								sprite: new_sprite,
								animation: new_animation
							};

							main.modified = true;
						}
					}
				}
			}
		},

		TagMessage::SetPreviewSprite(new_sprite_name) => {
			let sprite_index = lookup_file_index(&main.files, &new_sprite_name);
			if let Some(Tag::Agent(agent_tag)) = main.get_selected_tag_mut() {
				if let Some(sprite_index) = sprite_index {
					if let Preview::Manual{ sprite, animation: _ } = &mut agent_tag.preview {
						*sprite = sprite_index;
						main.modified = true;
					}
				}
			}
		},

		TagMessage::SetPreviewAnimation(new_animation) => {
			if let Some(Tag::Agent(agent_tag)) = main.get_selected_tag_mut() {
				if let Preview::Manual{ sprite: _, animation } = &mut agent_tag.preview {
					*animation = new_animation;
					main.modified = true;
				}
			}
		},

		TagMessage::SetEggPreviewType(use_manual_preview) => {
			let files = main.files.clone();
			if let Some(Tag::Egg(egg_tag)) = main.get_selected_tag_mut() {
				if let EggPreview::Manual{ sprite_male: _, sprite_female: _, animation: _ } = egg_tag.preview {
					egg_tag.preview_backup = egg_tag.preview.clone();
				}

				match use_manual_preview {
					false => {
						egg_tag.preview = EggPreview::None;
						main.modified = true;
					},

					true => {
						match egg_tag.get_first_sprite(&files) {
							Some(sprite_index) => {
								let mut new_sprite_male = sprite_index.clone();
								let mut new_sprite_female = sprite_index.clone();
								let mut new_animation = "0".to_string();

								if let EggPreview::Manual{ sprite_male, sprite_female, animation } = &egg_tag.preview_backup {
									if egg_tag.has_sprite(*sprite_male) { new_sprite_male = *sprite_male; }
									if egg_tag.has_sprite(*sprite_female) { new_sprite_female = *sprite_female; }
									new_animation = animation.to_string();
								}

								egg_tag.preview = EggPreview::Manual{
									sprite_male: new_sprite_male,
									sprite_female: new_sprite_female,
									animation: new_animation
								};

								main.modified = true;
							},

							None => {
								main.add_alert(&"ERROR: Must have at least one sprite file to enable preview".to_string(), true);
								return;
							}
						}
					}
				}
			}
		},

		TagMessage::SetEggPreviewSpriteMale(new_sprite_name) => {
			let sprite_index = lookup_file_index(&main.files, &new_sprite_name);
			if let Some(Tag::Egg(egg_tag)) = main.get_selected_tag_mut() {
				if let Some(sprite_index) = sprite_index {
					if let EggPreview::Manual{ sprite_male, sprite_female: _, animation: _ } = &mut egg_tag.preview {
						*sprite_male = sprite_index;
						main.modified = true;
					}
				}
			}
		},

		TagMessage::SetEggPreviewSpriteFemale(new_sprite_name) => {
			let sprite_index = lookup_file_index(&main.files, &new_sprite_name);
			if let Some(Tag::Egg(egg_tag)) = main.get_selected_tag_mut() {
				if let Some(sprite_index) = sprite_index {
					if let EggPreview::Manual{ sprite_male: _, sprite_female, animation: _ } = &mut egg_tag.preview {
						*sprite_female = sprite_index;
						main.modified = true;
					}
				}
			}
		},

		TagMessage::SetEggPreviewAnimation(new_animation) => {
			if let Some(Tag::Egg(egg_tag)) = main.get_selected_tag_mut() {
				if let EggPreview::Manual{ sprite_male: _, sprite_female: _, animation } = &mut egg_tag.preview {
					*animation = new_animation;
					main.modified = true;
				}
			}
		},

		TagMessage::SetRemoveScriptType(new_type) => {
			if let Some(Tag::Agent(agent_tag)) = main.get_selected_tag_mut() {
				if let RemoveScript::Manual(_remove_script_string) = &agent_tag.remove_script {
					agent_tag.remove_script_backup = agent_tag.remove_script.clone();
				}
				match new_type {
					0 => agent_tag.remove_script = RemoveScript::None,
					1 => agent_tag.remove_script = RemoveScript::Auto,
					_ => {
						match &agent_tag.remove_script_backup {
							RemoveScript::Manual(_remove_script_string) => {
								agent_tag.remove_script = agent_tag.remove_script_backup.clone();
							},
							_ => {
								agent_tag.remove_script = RemoveScript::Manual("".to_string());
							}
						}
					}
				}
				main.modified = true;
			}
		},

		TagMessage::SetRemoveScript(new_remove_script_string) => {
			if let Some(Tag::Agent(agent_tag)) = main.get_selected_tag_mut() {
				agent_tag.remove_script = RemoveScript::Manual(new_remove_script_string);
				main.modified = true;
			}
		},

		TagMessage::SetGenome(new_genome_name) => {
			let genetics_index = lookup_file_index(&main.files, &format!("{}.gen", new_genome_name));
			if let Some(Tag::Egg(egg_tag)) = main.get_selected_tag_mut() {
				if let Some(genetics_index) = genetics_index {
					egg_tag.genome = Some(genetics_index);
					main.modified = true;
				}
			}
		},

		TagMessage::SelectFile(filetype, index) => {
			match filetype {
				FileType::Script => { main.selection = Selection::Script(index) },
				FileType::Sprite => { main.selection = Selection::Sprite(index) },
				FileType::Sound => { main.selection = Selection::Sound(index) },
				FileType::Catalogue => { main.selection = Selection::Catalogue(index) },
				FileType::BodyData => { main.selection = Selection::BodyData(index) }
				FileType::Genetics => { main.selection = Selection::Genetics(index) },
			}
		},

		TagMessage::MoveFileUp(filetype, index) => {
			if let Some(tag) = main.get_selected_tag_mut() {
				let new_index = tag.move_file_up(&filetype, index);
				match filetype {
					FileType::Script => { main.selection = Selection::Script(new_index) },
					FileType::Sprite => { main.selection = Selection::Sprite(new_index) },
					FileType::Sound => { main.selection = Selection::Sound(new_index) },
					FileType::Catalogue => { main.selection = Selection::Catalogue(new_index) },
					FileType::BodyData => { main.selection = Selection::BodyData(new_index) }
					FileType::Genetics => { main.selection = Selection::Genetics(new_index) },
				}
				main.modified = true;
			}
		},

		TagMessage::MoveFileDown(filetype, index) => {
			if let Some(tag) = main.get_selected_tag_mut() {
				let new_index = tag.move_file_down(&filetype, index);
				match filetype {
					FileType::Script => { main.selection = Selection::Script(new_index) },
					FileType::Sprite => { main.selection = Selection::Sprite(new_index) },
					FileType::Sound => { main.selection = Selection::Sound(new_index) },
					FileType::Catalogue => { main.selection = Selection::Catalogue(new_index) },
					FileType::BodyData => { main.selection = Selection::BodyData(new_index) }
					FileType::Genetics => { main.selection = Selection::Genetics(new_index) },
				}
				main.modified = true;
			}
		},

		TagMessage::RemoveFile(filetype, index, filename) => {
			if confirm_remove_item(filename.as_str()) {
				if let Some(tag) = main.get_selected_tag_mut() {

					match tag {
						Tag::Agent(agent_tag) => {
							if let FileType::Sprite = filetype {
								if let Some(sprite_file_index) = agent_tag.sprites.get(index) {
									if let Preview::Manual{ sprite, animation: _ } = agent_tag.preview {
										if *sprite_file_index == sprite {
											agent_tag.preview_backup = agent_tag.preview.clone();
											agent_tag.preview = Preview::None;
										}
									}
								}
							}
						},

						Tag::Egg(egg_tag) => {
							if let FileType::Sprite = filetype {
								if let Some(sprite_file_index) = egg_tag.sprites.get(index) {
									if let EggPreview::Manual{ sprite_male, sprite_female, animation: _ } = egg_tag.preview {
										if *sprite_file_index == sprite_male || *sprite_file_index == sprite_female {
											egg_tag.preview_backup = egg_tag.preview.clone();
											egg_tag.preview = EggPreview::None;
										}
									}
								}
							}

							if let FileType::Genetics = filetype {
								if let Some(genetics_file_index) = egg_tag.genetics.get(index) {
									if let Some(genome) = egg_tag.genome {
										if *genetics_file_index == genome {
											egg_tag.genome = None;
										}
									}
								}
							}
						},

						_ => ()
					}

					tag.remove_file(&filetype, index.clone());
					main.modified = true;
				}
			}
		},

		TagMessage::AddFile => {
			add_file(main);
		},

		TagMessage::AddExistingFile(file_index) => {
			main.is_adding_existing_file = false;
			add_existing_file(main, file_index);
		},

		TagMessage::AddInlineCatalogue => {
			let catalogue_index = main.files.len();
			if let Some(Tag::Agent(tag)) = main.get_selected_tag_mut() {
				if let Ok(mut catalogue_file) = Catalogue::new(&tag.name) {
					catalogue_file.add_entry(CatalogueEntry{
						name: tag.name.to_string(),
						classifier: "".to_string(),
						description: tag.description.to_string()
					});
					if let Ok(()) = catalogue_file.fetch_data(&"".to_string()) {
						tag.catalogues.push(catalogue_index);
						main.files.push(CreaturesFile::Catalogue(catalogue_file));
					}
				}
			}
		}
	}
}

fn finish_adding_tag(main: &mut Main) {
	main.is_adding_new_tag = false;
	main.tags.sort_by_key(|t| t.get_name().clone());
	main.selected_tag_index = Some(main.tags.len() - 1);
	main.selection = Selection::None;
	main.modified = true;
	if main.filename.is_empty() {
		main.filename = "untitled.the".to_string();
	}
}

pub fn add_file(main: &mut Main) {
	let file = FileDialog::new()
		.add_filter("Creatures Files", &["cos", "c16", "blk", "wav", "catalogue", "png", "gen", "gno", "att"])
		.set_directory(&main.path)
		.pick_file();
	if let Some(filepath) = file {
		add_file_from_path(main, filepath.to_string_lossy().into_owned(), false);
	}
}

pub fn add_file_from_path(main: &mut Main, filepath: String, from_drop: bool) {
	let path = file_helper::path(&filepath);
	let filename = file_helper::filename(&filepath);
	let extension = file_helper::extension(&filepath);

	if main.filename.is_empty() {
		main.filename = "untitled.the".to_string();
	}
	if main.path.is_empty() {
		main.path = path.to_string();
	}

	match path.strip_prefix(&main.path) {
		Some(relative_path) => {
			let input_filename = format!("{}{}", relative_path, filename);

			match fs::read(&filepath) {
				Ok(contents) => {
					if from_drop {
						if let Selection::Sprite(sprite_index) = main.selection {
							if let Some(tag) = main.get_selected_tag_mut() {
								if let Some(sprite_list) = tag.get_file_list(&FileType::Sprite) {
									if let Some(sprite_file_index) = sprite_list.get(sprite_index) {
										let sprite_file_index = *sprite_file_index;
										if let Some(CreaturesFile::Sprite(sprite_file)) = main.files.get_mut(sprite_file_index) {
											if let Ok(sprite_frame) = SpriteFrame::new_from_data(&input_filename, &mut Bytes::from(contents.clone())) {
												if sprite_file.add_frame(sprite_frame) {
													return;
												}
											}
										}
									}
								}
							}
						}
					}

					if let Some(Tag::Egg(egg_tag)) = main.get_selected_tag_mut() {
						if let Some(genetics_file_index) = egg_tag.genome {
							if let Some(genetics_file) = main.files.get(genetics_file_index) {
								if (extension == "gen" || extension == "gno") && file_helper::title(&input_filename) != genetics_file.get_title() {
									main.add_alert(&"ERROR: Egg tags can only have one GEN file and one GNO, and they must have the same name".to_string(), true);
									return;
								}
							}
						}
					}

					match file_from_data(&extension, &input_filename, &mut Bytes::from(contents)) {
						Ok(file) => {
							let filetype = file.get_filetype();
							let mut file_index: Option<usize> = None;

							for (i, existing_file) in main.files.iter().enumerate() {
								if existing_file.get_input_filename() == input_filename {
									file_index = Some(i);
								}
							}

							if let None = file_index {
								main.files.push(file);
								file_index = Some(main.files.len() - 1);
							}

							if let Some(file_index) = file_index {
								add_existing_file(main, file_index);
							}
						},
						Err(why) => {
							main.add_alert(&format!("ERROR: Unable to add {}: {}", filename, why), true);
						}
					}
				},
				Err(why) => {
					main.add_alert(&format!("ERROR: Unable to open {}: {}", filename, why), true);
				}
			}
		},

		None => {
			alert_wrong_folder();
		}
	}
}

fn add_existing_file(main: &mut Main, file_index: usize) -> bool {
	let file = main.files.get(file_index);
	match file {
		Some(file) => {
			let filename = file.get_output_filename();
			let filetype = file.get_filetype();
			let extension = file.get_extension();
			let mut file_added = false;
			if let Some(tag) = main.get_selected_tag_mut() {
				if !tag.has_file(&filetype, file_index) {
					match tag.add_file(&filetype, file_index) {
						Some(index) => {
							match filetype {
								FileType::Script => { main.selection = Selection::Script(index); },
								FileType::Sprite => { main.selection = Selection::Sprite(index); },
								FileType::Sound => { main.selection = Selection::Sound(index); },
								FileType::Catalogue => { main.selection = Selection::Catalogue(index); },
								FileType::BodyData => { main.selection = Selection::BodyData(index); }
								FileType::Genetics => { main.selection = Selection::Genetics(index); },
							}
							file_added = true;
							main.modified = true;
						},
						None => {
							main.add_alert(&format!("ERROR: Unable to add {}: wrong filetype for this tag", filename), true);
						}
					}
				}
			}

			if file_added {
				if let Some(Tag::Egg(egg_tag)) = main.get_selected_tag_mut() {
					if let None = egg_tag.genome {
						if extension == "gen" {
							egg_tag.genome = Some(file_index);
						}
					}
				}
			}
			file_added
		},
		None => {
			// :(
			main.add_alert(&format!("ERROR: Unable to add file index {} as it no longer exists.", file_index), true);
			false
		}
	}
}

fn file_from_data(extension: &String, filename: &String, contents: &mut Bytes) -> Result<CreaturesFile, Box<dyn Error>> {
	match extension.as_str() {
		"cos" => Ok(CreaturesFile::Script(Script::new_from_data(filename, contents)?)),
		"c16" => Ok(CreaturesFile::Sprite(Sprite::new_from_data_raw(filename, contents)?)),
		"s16" => Ok(CreaturesFile::Sprite(Sprite::new_from_data_raw(filename, contents)?)),
		"blk" => Ok(CreaturesFile::Sprite(Sprite::new_from_data_raw(filename, contents)?)),
		"png" => Ok(CreaturesFile::Sprite(Sprite::new_from_data(filename, contents)?)),
		"wav" => Ok(CreaturesFile::Sound(Sound::new_from_data(filename, contents)?)),
		"catalogue" => Ok(CreaturesFile::Catalogue(Catalogue::new_from_data(filename, contents)?)),
		"att" => Ok(CreaturesFile::BodyData(BodyData::new_from_data(filename, contents)?)),
		"gen" => Ok(CreaturesFile::Genetics(Genetics::new_from_data(filename, contents)?)),
		"gno" => Ok(CreaturesFile::Genetics(Genetics::new_from_data(filename, contents)?)),
		_ => Err(create_error("File is not a valid creatures file type"))
	}
}
