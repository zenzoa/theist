use crate::agent;
use crate::pray;

use std::fs;
use std::str;
use std::path::PathBuf;
use rfd::{ FileDialog, MessageDialog, MessageLevel, MessageButtons };
use iced::widget::{ row, column, button, pick_list, horizontal_space, horizontal_rule };
use iced::{ Alignment, Length, Element, Sandbox, Settings };

enum Alert {
	Update(String),
	Error(String)
}

pub struct Main {
	filename: String,
	path: String,
	tags: Vec<agent::Tag>,
	files: Vec<agent::FileData>,
	modified: bool,
	alerts: Vec<Alert>
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
	NewFile,
	OpenFile,
	Save,
	SaveAs,
	Compile,
	AddTag,
	AddFile
}

impl Sandbox for Main {
	type Message = Message;

	fn new() -> Self {
		Self {
			filename: String::from("untitled.the"),
			path: String::from(""),
			tags: Vec::new(),
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
			},
			_ => {
				println!("MESSAGE: {:?}", message);
			}
		}
	}

	fn view(&self) -> Element<Message> {
		column![
			row![
				button("New").on_press(Message::NewFile),
				button("Open").on_press(Message::OpenFile),
				button("Save").on_press(Message::Save),
				button("Save As").on_press(Message::SaveAs),
				horizontal_space(Length::Fill),
				button("Compile").on_press(Message::Compile)
			]
			.padding(10)
			.spacing(5),
			horizontal_rule(1)
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
							self.tags = agent::parse_source(&contents, &self.path);
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
