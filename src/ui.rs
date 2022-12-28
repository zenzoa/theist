pub mod tag_view;
pub mod script_view;
pub mod sprite_view;
pub mod background_view;
pub mod sound_view;
pub mod catalogue_view;

use crate::agent::*;
use crate::agent::tag::*;
use crate::agent::script::*;
use crate::agent::sprite::*;
use crate::agent::background::*;
use crate::agent::sound::*;
use crate::agent::catalogue::*;
use crate::pray;

use std::fs;
use std::str;
use std::path::{ Path, PathBuf };
use rfd::{ FileDialog, MessageDialog, MessageLevel, MessageButtons };
use iced::widget::{ row, column, text, button, scrollable, horizontal_space, horizontal_rule, vertical_rule };
use iced::{ Application, Command, Element, Event, executor, Length, subscription, Subscription, Theme, window };

enum SelectionType {
	Tag,
	Script(usize),
	Sprite(usize),
	Background(usize),
	Sound(usize),
	Catalogue(usize)
}

enum Alert {
	Update(String),
	Error(String)
}

pub struct Main {
	filename: String,
	path: String,
	tags: Vec<Tag>,
	selected_tag: Option<usize>,
	selection_type: SelectionType,
	files: Vec<FileData>,
	modified: bool,
	alerts: Vec<Alert>
}

#[derive(Debug, Clone)]
pub enum Message {
	EventOccurred(Event),

	NewFile,
	OpenFile,
	Save,
	SaveAs,
	Compile,

	AddTag,
	DeleteTag,
	SelectTag(Option<usize>),
	SetTagName(String),
	SetTagDescription(String),
	SetTagVersion(String),
	SetTagSupportedGame(usize),
	SetTagPreviewAuto(bool),
	SetTagPreviewSprite(String),
	SetTagPreviewAnimation(String),
	SetTagRemoveScriptAuto(bool),
	SetTagRemoveScript(String),
	AddFile,

	SelectScript(usize),
	DeleteScript(usize),
	MoveScriptUp(usize),
	MoveScriptDown(usize),
	SetScriptSupportedGame(usize),

	SelectSprite(usize),
	DeleteSprite(usize),
	MoveSpriteUp(usize),
	MoveSpriteDown(usize),
	SetSpriteName(String),
	ConvertSpriteToBackground,

	AddSpriteFrame,
	DeleteSpriteFrame(usize),
	MoveSpriteFrameUp(usize),
	MoveSpriteFrameDown(usize),

	SelectBackground(usize),
	DeleteBackground(usize),
	MoveBackgroundUp(usize),
	MoveBackgroundDown(usize),
	ConvertBackgroundToSprite,

	SelectSound(usize),
	MoveSoundUp(usize),
	MoveSoundDown(usize),
	DeleteSound(usize),

	AddInlineCatalogue,
	SelectCatalogue(usize),
	DeleteCatalogue(usize),
	MoveCatalogueUp(usize),
	MoveCatalogueDown(usize),
	SetCatalogueName(String),

	AddCatalogueEntry,
	DeleteCatalogueEntry(usize),
	MoveCatalogueEntryUp(usize),
	MoveCatalogueEntryDown(usize),
	SetCatalogueEntryClassifier(String),
	SetCatalogueEntryName(String),
	SetCatalogueEntryDescription(String)
}

impl Application for Main {
	type Message = Message;
	type Theme = Theme;
	type Executor = executor::Default;
	type Flags = ();

	fn new(_flags: ()) -> (Self, Command<Message>) {
		(Self {
			filename: String::from("untitled.the"),
			path: String::from(""),
			tags: Vec::new(),
			selected_tag: None,
			selection_type: SelectionType::Tag,
			files: Vec::new(),
			modified: false,
			alerts: Vec::new()
		}, Command::none())
	}

	fn title(&self) -> String {
		if self.filename.is_empty() {
			String::from("Theist")
		} else if self.modified {
			format!("Theist - {}*", &self.filename)
		} else {
			format!("Theist - {}", &self.filename)
		}
	}

	fn update(&mut self, message: Message) -> Command<Message> {
		match message {
			Message::EventOccurred(event) => {
				if let Event::Window(window::Event::FileDropped(path)) = event {
					self.add_file_from_path(path, true);
				}
			},
			Message::NewFile => {
				if !self.modified || confirm_discard_changes() {
					self.filename = String::from("untitled.the");
					self.path = String::from("");
					self.tags = Vec::new();
					self.modified = false;
				}
			},
			Message::OpenFile => {
				if !self.modified || confirm_discard_changes() {
					let file = FileDialog::new()
						.add_filter("theist", &["the", "txt"])
						.add_filter("agent", &["agent", "agents"])
						.set_directory(&self.path)
						.pick_file();
					if let Some(path) = file {
						self.open(path);
					}
				}
			},
			Message::Save => {
				if self.path.is_empty() {
					let file = FileDialog::new()
						.set_directory(&self.path)
						.set_file_name(&self.filename)
						.save_file();
					if let Some(path) = file {
						self.save(path);
						self.modified = false;
					}
				} else {
					self.save(PathBuf::from(format!("{}{}", &self.path, &self.filename)));
					self.modified = false;
				}
			},
			Message::SaveAs => {
				let file = FileDialog::new()
					.set_directory(&self.path)
					.set_file_name(&self.filename)
					.save_file();
				if let Some(path) = file {
					self.save(path);
					self.modified = false;
				}
			},
			Message::AddTag => {
				let mut new_tag = AgentTag::new();
				new_tag.name = String::from("My Agent");
				self.tags.push(Tag::Agent(new_tag));
				self.selected_tag = Some(self.tags.len() - 1);
				self.modified = true;
			},
			Message::DeleteTag => {
				if confirm_delete_tag() {
					if let Some(selected_tag) = &self.selected_tag {
						if selected_tag < &self.tags.len() {
							self.tags.remove(*selected_tag);
							self.selected_tag = if self.tags.is_empty() {
								None
							} else if selected_tag > &0 {
								Some(selected_tag - 1)
							} else {
								Some(0)
							};
							self.modified = true;
						}
					}
				}
			}
			Message::SelectTag(selected_tag) => {
				self.selected_tag = selected_tag;
				self.selection_type = SelectionType::Tag;
			},
			Message::SetTagName(new_name) => {
				if let Some(selected_tag) = self.selected_tag {
					self.tags[selected_tag].set_name(new_name);
					self.modified = true;
				}
			},
			Message::SetTagDescription(new_description) => {
				if let Some(selected_tag) = self.selected_tag {
					self.tags[selected_tag].set_description(new_description);
					self.modified = true;
				}
			},
			Message::SetTagVersion(new_version) => {
				if let Some(selected_tag) = self.selected_tag {
					self.tags[selected_tag].set_version(new_version);
					self.modified = true;
				}
			},
			Message::SetTagSupportedGame(new_supported_game) => {
				if let Some(selected_tag) = self.selected_tag {
					self.tags[selected_tag].set_supported_game(new_supported_game);
					self.modified = true;
				}
			},
			Message::SetTagPreviewAuto(is_auto) => {
				if let Some(selected_tag) = self.selected_tag {
					self.tags[selected_tag].set_preview_auto(is_auto);
					self.modified = true;
				}
			},
			Message::SetTagPreviewSprite(new_sprite) => {
				if let Some(selected_tag) = self.selected_tag {
					self.tags[selected_tag].set_preview_sprite(new_sprite);
					self.modified = true;
				}
			},
			Message::SetTagPreviewAnimation(new_animation) => {
				if let Some(selected_tag) = self.selected_tag {
					self.tags[selected_tag].set_preview_animation(new_animation);
					self.modified = true;
				}
			},
			Message::SetTagRemoveScriptAuto(is_auto) => {
				if let Some(selected_tag) = self.selected_tag {
					self.tags[selected_tag].set_removescript_auto(is_auto);
					self.modified = true;
				}
			},
			Message::SetTagRemoveScript(new_removescript) => {
				if let Some(selected_tag) = self.selected_tag {
					self.tags[selected_tag].set_removescript_string(new_removescript);
					self.modified = true;
				}
			},
			Message::AddFile => {
				self.add_file();
			},
			Message::SelectScript(index) => {
				self.selection_type = SelectionType::Script(index);
			},
			Message::SelectSprite(index) => {
				self.selection_type = SelectionType::Sprite(index);
			},
			Message::SelectBackground(index) => {
				self.selection_type = SelectionType::Background(index);
			},
			Message::SelectSound(index) => {
				self.selection_type = SelectionType::Sound(index);
			},
			Message::SelectCatalogue(index) => {
				self.selection_type = SelectionType::Catalogue(index);
			},
			_ => {
				println!("MESSAGE: {:?}", message);
			}
		}
		Command::none()
	}

	fn subscription(&self) -> Subscription<Message> {
		subscription::events().map(Message::EventOccurred)
	}

	fn view(&self) -> Element<Message> {
		let toolbar = row![
			button("New").on_press(Message::NewFile),
			button("Open").on_press(Message::OpenFile),
			button("Save").on_press(Message::Save),
			button("Save As").on_press(Message::SaveAs),
			horizontal_space(Length::Fill),
			button("Compile").on_press(Message::Compile)
		].padding(10).spacing(5);

		let mut tabs = row![].spacing(5);

		for (i, tag) in self.tags.iter().enumerate() {
			let tag_name = match tag {
				Tag::Agent(agent_tag) => &agent_tag.name,
				_ => ""
			};
			tabs = tabs.push(
				button(tag_name)
					.on_press(Message::SelectTag(Some(i)))
					.width(Length::FillPortion(1))
			);
		}

		tabs = tabs.push(button("+").on_press(Message::AddTag));

		let mut tab_contents = column![];
		let mut current_properties = column![];

		if let Some(selected_tag) = self.selected_tag {
			if let Some(tag) = self.tags.get(selected_tag) {
				match tag {
					Tag::Agent(tag) => {
						tab_contents = tag_view::agent_listing(tag);
						match self.selection_type {
							SelectionType::Script(index) => {
								if let Some(script) = tag.scripts.get(index) {
									current_properties = script_view::properties(script);
								}
							},
							SelectionType::Sprite(index) => {
								if let Some(sprite) = tag.sprites.get(index) {
									current_properties = sprite_view::properties(sprite);
								}
							},
							SelectionType::Background(index) => {
								if let Some(background) = tag.backgrounds.get(index) {
									current_properties = background_view::properties(background);
								}
							},
							SelectionType::Sound(index) => {
								if let Some(sound) = tag.sounds.get(index) {
									current_properties = sound_view::properties(sound);
								}
							},
							SelectionType::Catalogue(index) => {
								if let Some(catalogue) = tag.catalogues.get(index) {
									current_properties = catalogue_view::properties(catalogue);
								}
							},
							_ => {
								current_properties = tag_view::agent_properties(tag);
							}
						}
					},
					_ => ()
				}
			}
		}

		let main_pane = column![
			tabs.padding([20, 20, 0, 20]),
			scrollable(
				tab_contents.padding(20)
			).height(Length::Fill)
		].width(Length::FillPortion(3));

		let properties_pane = column![
			current_properties
		].spacing(5).width(Length::FillPortion(2));

		let mut alerts_pane = column![
			text("Alerts")
		].padding(10).spacing(5);

		for alert in &self.alerts {
			match alert {
				Alert::Update(message) => {
					alerts_pane = alerts_pane.push(text(&message));
				},
				Alert::Error(message) => {
					alerts_pane = alerts_pane.push(text(&message));
				}
			}
		}

		column![
			toolbar,
			horizontal_rule(1),
			row![
					main_pane,
					vertical_rule(1),
					properties_pane
				].height(Length::Fill),
			horizontal_rule(1),
			alerts_pane
		].into()
	}

}

impl Main {
	fn add_alert(&mut self, contents: &str, is_error: bool) {
		self.alerts.push(
			if is_error {
				Alert::Error(contents.to_string())
			} else {
				Alert::Update(contents.to_string())
			}
		);
	}

	fn set_path_and_name(&mut self, path: &Path) {
		self.path = match path.parent() {
			Some(parent) => parent.to_string_lossy().into_owned() + "/",
			None => String::from("")
		};
		self.filename = match path.file_name() {
			Some(filename) => filename.to_string_lossy().into_owned(),
			None => String::from("untitled.the")
		};
	}

	fn open(&mut self, path: PathBuf) {
		self.set_path_and_name(&path);
		let extension = match path.extension() {
			Some(extension) => extension.to_string_lossy().into_owned(),
			None => String::from("")
		};

		match fs::read(format!("{}{}", &self.path, &self.filename)) {
			Ok(contents) => {
				if extension == "agent" || extension == "agents" {
					match pray::decode(&contents) {
						Ok((tags, files)) => {
							self.tags = tags;
							self.files = files;
						},
						Err(why) => {
							self.add_alert("Unable to understand file", true);
							println!("ERROR: Unable to understand file: {}", why);
						}
					}
				} else {
					match str::from_utf8(&contents) {
						Ok(contents) => {
							self.tags = parse_source(contents, &self.path);
							// TODO - parse_source should send back any alerts
							if self.tags.is_empty() {
								self.add_alert("No tags found in file", true);
							}
						},
						Err(why) => {
							self.add_alert("Unable to understand file", true);
							println!("ERROR: Unable to understand file: {}", why);
						}
					}
				}
				self.modified = false;
				if self.tags.is_empty() {
					self.selected_tag = None;
				} else {
					self.selected_tag = Some(0);
				}
			},
			Err(why) => {
				self.add_alert("Unable to open file", true);
				println!("ERROR: Unable to open file: {}", why);
			}
		}
	}

	fn save(&mut self, path: PathBuf) {
		self.set_path_and_name(&path);
		// TODO: save theist source file
		// TODO: save any files loaded locally but not yet in the path
	}

	fn add_file(&mut self) {
		let file = FileDialog::new()
			.add_filter("Creatures Files", &["cos", "c16", "blk", "wav", "catalogue", "png"])
			.set_directory(&self.path)
			.pick_file();
		if let Some(file_path) = file {
			self.add_file_from_path(file_path, false);
		}
	}

	fn add_file_from_path(&mut self, file_path: PathBuf, allow_file_open: bool) {
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

		if allow_file_open
			&& (extension == "the" || extension == "txt" || extension == "agent" || extension == "agents")
			&& (!self.modified || confirm_discard_changes()) {
				self.open(file_path);
				return;
		}

		if self.path.is_empty() {
			self.path = path;
		} else if self.path != path {
			alert_wrong_folder();
			return;
		}

		if let Some(selected_tag) = self.selected_tag {
			match &mut self.tags[selected_tag] {
				Tag::Agent(tag) => {
					if tag.filepath.is_empty() {
						tag.filepath = self.path.clone();
					}
					match extension.as_str() {
						"cos" => {
							tag.scripts.push(Script::new(&filename, &tag.supported_game.to_string()));
						},
						"c16" => {
							tag.sprites.push(Sprite::new(&filename));
						},
						"blk" => {
							tag.backgrounds.push(Background::new(&filename));
						},
						"wav" => {
							tag.sounds.push(Sound::new(&filename));
						},
						"catalogue" => {
							tag.catalogues.push(Catalogue::new(&filename));
						},
						"png" => {
							let mut sprite = Sprite::new(format!("{}.c16", &title).as_str());
							let frame = SpriteFrame::new(&filename);
							sprite.add_frame(frame);
							tag.sprites.push(sprite);
						},
						_ => {
							// TODO: alert user that they picked an invalid file type for an agent tag
						}
					}
					self.modified = true;
				},
				_ => ()
			}
		}
	}
}

fn confirm_discard_changes() -> bool {
	MessageDialog::new()
		.set_title("File modified")
		.set_description("Do you want to continue anyway and lose any unsaved work?")
		.set_level(MessageLevel::Warning)
		.set_buttons(MessageButtons::OkCancel)
		.show()
}

fn confirm_delete_tag() -> bool {
	MessageDialog::new()
		.set_title("Delete tag")
		.set_description("Are you sure you want to delete this tag? It won't delete any files it refers to, but you will lose all info stored in the tag itself.")
		.set_level(MessageLevel::Warning)
		.set_buttons(MessageButtons::OkCancel)
		.show()
}

fn alert_wrong_folder() -> bool{
	MessageDialog::new()
		.set_title("Wrong folder")
		.set_description("Unable to load file. All files must be located in the same folder.")
		.set_level(MessageLevel::Warning)
		.set_buttons(MessageButtons::Ok)
		.show()
}
