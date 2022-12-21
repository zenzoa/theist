use crate::agent;

use std::path::PathBuf;
use rfd::{ FileDialog, MessageDialog, MessageLevel, MessageButtons };
use iced::widget::{ row, column, button, pick_list, horizontal_space, horizontal_rule };
use iced::{ Alignment, Length, Element, Sandbox, Settings };

pub struct Main {
	filename: String,
	path: String,
	tags: Vec<agent::Tag>,
	files: Vec<Option<agent::FileData>>,
	modified: bool
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
			modified: false
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
					println!("{:?}", path)
					// set name to filename
					// set path to path
					// if .agent(s), decompile into tags + files
					// if .the/.txt, parse into tags, grab files (show warning for files not found)
				}
			},
			Message::Save => {
				if self.path.is_empty() {
					let file = FileDialog::new()
						.set_directory(&self.path)
						.set_file_name(&self.filename)
						.save_file();
					if let Some(path) = file {
						println!("{:?}", path);
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
					println!("{:?}", path);
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
	fn save(&mut self, path: PathBuf) {
		self.path = match path.parent() {
			Some(parent) => parent.to_string_lossy().into_owned() + "/",
			None => String::from("") + "/"
		};
		self.filename = match path.file_name() {
			Some(filename) => filename.to_string_lossy().into_owned(),
			None => String::from("untitled.the")
		};
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
