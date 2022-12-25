use crate::agent::*;
use crate::pray;

use std::fs;
use std::str;
use std::path::PathBuf;
use rfd::{ FileDialog, MessageDialog, MessageLevel, MessageButtons };
use iced::widget::{ container, row, column, text, text_input, button, radio, checkbox, horizontal_space, horizontal_rule, vertical_space, vertical_rule };
use iced::{ Alignment, Length, Element, Sandbox, Settings };

enum Alert {
	Update(String),
	Error(String)
}

pub struct Main {
	filename: String,
	path: String,
	tags: Vec<Tag>,
	selected_tag: Option<usize>,
	files: Vec<FileData>,
	modified: bool,
	alerts: Vec<Alert>
}

#[derive(Debug, Clone)]
pub enum Message {
	NewFile,
	OpenFile,
	Save,
	SaveAs,
	Compile,

	AddFile,

	AddTag,
	DeleteTag,
	SelectTag(Option<usize>),
	ChangeTagName(String),
	ChangeTagDescription(String),
	ChangeTagVersion(String),
	ChangeTagSupportedGame(usize),
	ChangeTagInjectorPreviewAuto(bool),
	ChangeTagInjectorPreviewSprite(String),
	ChangeTagInjectorPreviewAnimation(String),
	ChangeTagRemoveScriptAuto(bool),
	ChangeTagRemoveScript(String)
}

impl Sandbox for Main {
	type Message = Message;

	fn new() -> Self {
		Self {
			filename: String::from("untitled.the"),
			path: String::from(""),
			tags: Vec::new(),
			selected_tag: None,
			files: Vec::new(),
			modified: false,
			alerts: Vec::new()
		}
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

	fn update(&mut self, message: Message) {
		match message {
			Message::NewFile => {
				if self.modified && !confirm_discard_changes() {
					return;
				}
				self.filename = String::from("untitled.the");
				self.path = String::from("");
				self.tags = Vec::new();
				self.modified = false;
			},
			Message::OpenFile => {
				if self.modified && !confirm_discard_changes() {
					return;
				}
				let file = FileDialog::new()
					.add_filter("theist", &["the", "txt"])
					.add_filter("agent", &["agent", "agents"])
					.set_directory(&self.path)
					.pick_file();
				if let Some(path) = file {
					println!("Open: {:?}", &path);
					self.open(path);
					self.modified = false;
					if self.tags.is_empty() {
						self.selected_tag = None;
					} else {
						self.selected_tag = Some(0);
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
						println!("Save: {:?}", &path);
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
					println!("Save As: {:?}", &path);
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
				self.modified = true;
			},
			Message::ChangeTagName(name) => {
				if let Some(selected_tag) = self.selected_tag {
					match &mut self.tags[selected_tag] {
						Tag::Agent(tag) => tag.name = name,
						_ => ()
					}
				}
				self.modified = true;
			},
			Message::ChangeTagDescription(description) => {
				if let Some(selected_tag) = self.selected_tag {
					match &mut self.tags[selected_tag] {
						Tag::Agent(tag) => tag.description = description,
						_ => ()
					}
				}
				self.modified = true;
			},
			Message::ChangeTagVersion(version) => {
				if let Some(selected_tag) = self.selected_tag {
					match &mut self.tags[selected_tag] {
						Tag::Agent(tag) => tag.version = version,
						_ => ()
					}
				}
				self.modified = true;
			},
			Message::ChangeTagSupportedGame(supported_game) => {
				if let Some(selected_tag) = self.selected_tag {
					match &mut self.tags[selected_tag] {
						Tag::Agent(tag) => tag.supported_game = match supported_game {
							1 => SupportedGame::C3,
							2 => SupportedGame::DS,
							_ => SupportedGame::C3DS
						},
						_ => ()
					}
				}
				self.modified = true;
			},
			Message::ChangeTagInjectorPreviewAuto(is_auto) => {
				if let Some(selected_tag) = self.selected_tag {
					match &mut self.tags[selected_tag] {
						Tag::Agent(tag) => {
							tag.injector_preview = if is_auto {
								InjectorPreview::Auto
							} else {
								InjectorPreview::Manual {
									sprite: String::from(""), // TODO: get name of first sprite
									animation: String::from("0")
								}
							};
						},
						_ => ()
					}
				}
				self.modified = true;
			},
			Message::ChangeTagInjectorPreviewSprite(new_sprite) => {
				if let Some(selected_tag) = self.selected_tag {
					match &mut self.tags[selected_tag] {
						Tag::Agent(tag) => {
							if let InjectorPreview::Manual{ sprite, animation } = &tag.injector_preview {
								tag.injector_preview = InjectorPreview::Manual{
									sprite: new_sprite,
									animation: animation.clone()
								}
							}
						},
						_ => ()
					}
				}
				self.modified = true;
			},
			Message::ChangeTagInjectorPreviewAnimation(new_animation) => {
				if let Some(selected_tag) = self.selected_tag {
					match &mut self.tags[selected_tag] {
						Tag::Agent(tag) => {
							if let InjectorPreview::Manual{ sprite, animation } = &tag.injector_preview {
								tag.injector_preview = InjectorPreview::Manual{
									sprite: sprite.clone(),
									animation: new_animation
								}
							}
						},
						_ => ()
					}
				}
				self.modified = true;
			},
			Message::ChangeTagRemoveScriptAuto(is_auto) => {
				if let Some(selected_tag) = self.selected_tag {
					match &mut self.tags[selected_tag] {
						Tag::Agent(tag) => {
							tag.remove_script = if is_auto {
								RemoveScript::Auto
							} else {
								RemoveScript::Manual(String::from(""))
							};
						},
						_ => ()
					}
				}
				self.modified = true;
			},
			Message::ChangeTagRemoveScript(remove_script) => {
				if let Some(selected_tag) = self.selected_tag {
					match &mut self.tags[selected_tag] {
						Tag::Agent(tag) => {
							tag.remove_script = RemoveScript::Manual(remove_script);
						},
						_ => ()
					}
				}
				self.modified = true;
			},
			Message::AddFile => {
				let file = FileDialog::new()
					.add_filter("image", &["png", "c16", "blk"])
					.add_filter("script", &["cos"])
					.add_filter("sound", &["wav"])
					.add_filter("catalogue", &["catalogue"])
					.add_filter("genetics", &["gen", "gno"])
					.set_directory(&self.path)
					.pick_file();
				if let Some(path) = file {
					println!("{:?}", path)
				}
				self.modified = true;
			},
			_ => {
				println!("MESSAGE: {:?}", message);
			}
		}
	}

	fn view(&self) -> Element<Message> {
		let toolbar = row![
				button("New").on_press(Message::NewFile),
				button("Open").on_press(Message::OpenFile),
				button("Save").on_press(Message::Save),
				button("Save As").on_press(Message::SaveAs),
				horizontal_space(Length::Fill),
				button("Compile").on_press(Message::Compile)
			]
			.padding(10)
			.spacing(5);

		let mut tabs = row![]
			.spacing(5);

		for (i, tag) in self.tags.iter().enumerate() {
			let tag_name = match tag {
				Tag::Agent(agent_tag) => &agent_tag.name,
				_ => ""
			};
			// if let Some(selected_tag) = self.selected_tag {
			// 	if selected_tag == i {
			// 		tag_name = "selected";
			// 	}
			// }
			tabs = tabs.push(
				button(tag_name)
					.on_press(Message::SelectTag(Some(i)))
					.width(Length::FillPortion(1))
			);
		}

		tabs = tabs.push(button("+").on_press(Message::AddTag));

		let mut tab_contents = column![]
			.spacing(10);

		let mut current_properties = column![]
			.spacing(10);

		if let Some(selected_tag) = self.selected_tag {
			if let Some(tag) = self.tags.get(selected_tag) {
				match tag {
					Tag::Agent(tag) => {
						let supported_game = match tag.supported_game {
							SupportedGame::C3DS => Some(0),
							SupportedGame::C3 => Some(1),
							SupportedGame::DS => Some(2)
						};
						current_properties = column![
							text(format!("Properties for {}", &tag.name)),
							row![
									text("Name").width(Length::FillPortion(1)),
									text_input("My Agent", &tag.name, Message::ChangeTagName).width(Length::FillPortion(3))
								]
								.spacing(5)
								.align_items(Alignment::Center),
							row![
									text("Description").width(Length::FillPortion(1)),
									text_input("Something that does some stuff", &tag.description, Message::ChangeTagDescription).width(Length::FillPortion(3))
								]
								.spacing(5)
								.align_items(Alignment::Center),
							row![
									text("Version").width(Length::FillPortion(1)),
									text_input("1.0", &tag.version, Message::ChangeTagVersion).width(Length::FillPortion(3))
								]
								.spacing(5)
								.align_items(Alignment::Center),
							row![
									text("Game").width(Length::FillPortion(1)),
									radio("C3 + DS", 0, supported_game, Message::ChangeTagSupportedGame).width(Length::FillPortion(1)),
									radio("C3 only", 1, supported_game, Message::ChangeTagSupportedGame).width(Length::FillPortion(1)),
									radio("DS only", 2, supported_game, Message::ChangeTagSupportedGame).width(Length::FillPortion(1))
								]
								.spacing(5)
								.align_items(Alignment::Center)
						]
						.spacing(20);

						current_properties = current_properties.push(
							row![
								text("Injector Preview"),
								checkbox("Auto", tag.injector_preview == InjectorPreview::Auto, Message::ChangeTagInjectorPreviewAuto)
							]
							.spacing(20)
							.align_items(Alignment::Center)
						);

						if let InjectorPreview::Manual { sprite, animation } = &tag.injector_preview {
							current_properties = current_properties.push(
								row![
									text_input("Sprite Name", sprite, Message::ChangeTagInjectorPreviewSprite),
									text_input("Animation String", animation, Message::ChangeTagInjectorPreviewAnimation)
								]
								.spacing(5)
								.align_items(Alignment::Center)
							);
						}

						current_properties = current_properties.push(
							row![
								text("Remove Script"),
								checkbox("Auto", tag.remove_script == RemoveScript::Auto, Message::ChangeTagRemoveScriptAuto)
							]
							.spacing(20)
							.align_items(Alignment::Center)
						);

						if tag.remove_script != RemoveScript::Auto {
							let remove_script = if let RemoveScript::Manual(remove_script) = &tag.remove_script {
								remove_script.clone().to_string()
							} else {
								String::from("")
							};
							current_properties = current_properties.push(
								text_input("Remove Script", &remove_script, Message::ChangeTagRemoveScript)
							);
						}

						current_properties = current_properties.push(vertical_space(Length::Fill));
						current_properties = current_properties.push(button("Delete").on_press(Message::DeleteTag));
					},
					_ => ()
				}
			}
		}

		let main_pane = column![
				tabs,
				tab_contents
			]
			.padding(20)
			.spacing(5)
			.width(Length::FillPortion(3));

		let properties_pane = column![
				current_properties
			]
			.padding(20)
			.spacing(5)
			.width(Length::FillPortion(2));

		let mut alerts_pane = column![
				text("Alerts")
			]
			.padding(10)
			.spacing(5);

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
				]
				.height(Length::Fill),
			horizontal_rule(1),
			alerts_pane
		]
		.into()
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

	fn set_path_and_name(&mut self, path: &PathBuf) {
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
							self.tags = parse_source(&contents, &self.path);
							// TODO - parse_source should send back any alerts
							if self.tags.len() == 0 {
								self.add_alert("No tags found in file", true);
							}
						},
						Err(why) => {
							self.add_alert("Unable to understand file", true);
							println!("ERROR: Unable to understand file: {}", why);
						}
					}
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
