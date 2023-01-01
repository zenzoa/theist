pub mod messages;
pub mod dialogs;
pub mod views;

use messages::{ Message, check_message };
use messages::file_message::FileMessage;
use messages::tag_message::TagMessage;
use views::*;

use crate::agent::*;
use crate::agent::tag::*;
use crate::agent::agent_tag::*;

use std::str;
use std::path::Path;
use iced::widget::{ row, column, text, button, scrollable, horizontal_space, horizontal_rule, vertical_rule };
use iced::{ Application, Command, Element, executor, Length, subscription, Subscription, Theme };

pub enum SelectionType {
	Tag,
	Script(usize),
	Sprite(usize),
	Background(usize),
	Sound(usize),
	Catalogue(usize)
}

pub enum Alert {
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
	alerts: Vec<Alert>,
	modified: bool,
	exit: bool
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
			tags: vec![ Tag::Agent(AgentTag::new(String::from("My Agent"))) ],
			selected_tag: Some(0),
			selection_type: SelectionType::Tag,
			files: Vec::new(),
			alerts: Vec::new(),
			modified: false,
			exit: false
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

	fn should_exit(&self) -> bool {
		self.exit
	}

	fn update(&mut self, message: Message) -> Command<Message> {
		check_message(self, message);
		Command::none()
	}

	fn subscription(&self) -> Subscription<Message> {
		subscription::events().map(Message::EventOccurred)
	}

	fn view(&self) -> Element<Message> {
		let toolbar = row![
			button("New").on_press(Message::File(FileMessage::New)),
			button("Open").on_press(Message::File(FileMessage::Open)),
			button("Save").on_press(Message::File(FileMessage::Save)),
			button("Save As").on_press(Message::File(FileMessage::SaveAs)),
			horizontal_space(Length::Fill),
			button("Compile").on_press(Message::File(FileMessage::Compile))
		].padding(10).spacing(5);

		let mut tabs = row![].spacing(5);

		for (i, tag) in self.tags.iter().enumerate() {
			let tag_name = match tag {
				Tag::Agent(agent_tag) => &agent_tag.name,
				_ => ""
			};
			tabs = tabs.push(
				button(tag_name)
					.on_press(Message::Tag(TagMessage::Select(Some(i))))
					.width(Length::FillPortion(1))
			);
		}

		tabs = tabs.push(button("+").on_press(Message::Tag(TagMessage::Add)));

		let mut tab_contents = column![];
		let mut current_properties = column![];

		if let Some(selected_tag) = self.selected_tag {
			if let Some(tag) = self.tags.get(selected_tag) {
				match tag {
					Tag::Agent(tag) => {
						tab_contents = agent_tag_view::listing(tag);
						match self.selection_type {
							SelectionType::Script(index) => {
								if let Some(script) = tag.scripts.get(index) {
									current_properties = script_view::properties(script);
								}
							},
							SelectionType::Sprite(index) => {
								if let Some(sprite) = tag.sprites.get(index) {
									current_properties = sprite_view::properties(sprite, true);
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
								current_properties = agent_tag_view::properties(tag);
							}
						}
					},
					Tag::Egg(tag) => {
						tab_contents = egg_tag_view::listing(tag);
						match self.selection_type {
							SelectionType::Sprite(index) => {
								if let Some(sprite) = tag.sprites.get(index) {
									current_properties = sprite_view::properties(sprite, false);
								}
							},
							_ => {
								current_properties = egg_tag_view::properties(tag);
							}
						}
					},
					Tag::Empty => ()
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
}
