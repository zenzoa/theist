pub mod alert;
pub mod dialog;
pub mod icon;
pub mod message;
pub mod modal;
pub mod view;

use alert::Alert;
use modal::Modal;
use message::{ Message, check_message };
use message::tag::TagMessage;
use view::alert_list;
use view::file_list;
use view::properties;
use view::tag_list;
use view::toolbar;
use crate::agent::file::CreaturesFile;
use crate::agent::tag::Tag;

use iced::widget::{ button, column, container, horizontal_rule, row, scrollable, text };
use iced::{ Application, Command, Element, executor, Length, subscription, Subscription, Theme, theme };

pub struct Main {
	filename: String,
	path: String,
	tags: Vec<Tag>,
	files: Vec<CreaturesFile>,
	selected_tag_index: Option<usize>,
	selection: Selection,
	alerts: Vec<Alert>,
	modified: bool,
	is_adding_new_tag: bool,
	is_adding_existing_file: bool
}

impl Application for Main {
	type Message = Message;
	type Theme = Theme;
	type Executor = executor::Default;
	type Flags = ();

	fn new(_flags: ()) -> (Self, Command<Message>) {
		(Self {
			filename: "".to_string(),
			path: "".to_string(),
			tags: Vec::new(),
			files: Vec::new(),
			selected_tag_index: None,
			selection: Selection::None,
			alerts: Vec::new(),
			modified: false,
			is_adding_new_tag: false,
			is_adding_existing_file: false
		}, Command::none())
	}

	fn title(&self) -> String {
		if self.filename.is_empty() {
			"Theist".to_string()
		} else if self.modified {
			format!("Theist - {}*", &self.filename)
		} else {
			format!("Theist - {}", &self.filename)
		}
	}

	fn update(&mut self, message: Message) -> Command<Message> {
		check_message(self, message)
	}

	fn subscription(&self) -> Subscription<Message> {
		subscription::events().map(Message::EventOccurred)
	}

	fn view(&self) -> Element<Message> {
		let content = column![

			toolbar::view(self),

			horizontal_rule(1),

			scrollable(
				tag_list::view(self)
			).horizontal_scroll(scrollable::Properties::new()),

			horizontal_rule(1),

			row![

				file_list::view(self)
					.height(Length::Fill)
					.width(Length::FillPortion(1)),

				container(scrollable(properties::view(self).padding(20)).height(Length::Fill))
					.width(Length::FillPortion(1))
					.height(Length::Fill)
					.style(theme::Container::Box)

			].height(Length::Fill),

			horizontal_rule(1),

			alert_list::view(self)

		];

		if self.is_adding_new_tag {
			let new_tag_modal = container(
				column![
					text("Add New Tag"),
					horizontal_rule(1),
					column![
						button("Agent Tag")
							.on_press(Message::Tag(TagMessage::AddAgentTag))
							.width(Length::Fill),
						button("Egg Tag")
							.on_press(Message::Tag(TagMessage::AddEggTag))
							.width(Length::Fill)
					].spacing(10),
					horizontal_rule(1),
					button("Cancel")
						.on_press(Message::HideNewTagDialog)
						.width(Length::Fill)
						.style(theme::Button::Secondary)
				].spacing(20)
			)
				.width(300)
				.padding(20)
				.style(theme::Container::Box);

			Modal::new(content, new_tag_modal)
				.on_blur(Message::HideNewTagDialog)
				.into()

		} else if self.is_adding_existing_file {
			let existing_file_modal = container(
				container(
					column![
						text("Add Existing File"),
						horizontal_rule(1),
						scrollable(file_list::existing_file_list(self))
							.height(Length::Fill),
						horizontal_rule(1),
						button("Cancel")
							.on_press(Message::HideExistingFileDialog)
							.width(Length::Fill)
							.style(theme::Button::Secondary)
					].spacing(20)
				)
					.width(500)
					.padding(20)
					.style(theme::Container::Box)
			).padding(20);

			Modal::new(content, existing_file_modal)
				.on_blur(Message::HideExistingFileDialog)
				.into()

		} else {
			content.into()
		}
	}
}

impl Main {
	pub fn add_alert(&mut self, contents: &String, is_error: bool) {
		self.alerts.push(
			if is_error {
				Alert::Error(contents.to_string())
			} else {
				Alert::Update(contents.to_string())
			}
		);
	}

	pub fn get_selected_tag(&self) -> Option<&Tag> {
		match self.selected_tag_index {
			Some(tag_index) => {
				self.tags.get(tag_index)
			},
			None => None
		}
	}

	pub fn get_selected_tag_mut(&mut self) -> Option<&mut Tag> {
		match self.selected_tag_index {
			Some(tag_index) => {
				self.tags.get_mut(tag_index)
			},
			None => None
		}
	}

	pub fn get_selected_file_index(&self) -> Option<usize> {
		match self.get_selected_tag() {
			Some(tag) => {
				match tag {
					Tag::Agent(agent_tag) => {
						match self.selection {
							Selection::Script(index) =>
								match agent_tag.scripts.get(index) {
									Some(file_index) => Some(file_index.clone()),
									None => None
								},
							Selection::Sprite(index) =>
								match agent_tag.sprites.get(index) {
									Some(file_index) => Some(file_index.clone()),
									None => None
								},
							Selection::Sound(index) =>
								match agent_tag.sounds.get(index) {
									Some(file_index) => Some(file_index.clone()),
									None => None
								},
							Selection::Catalogue(index) =>
								match agent_tag.catalogues.get(index) {
									Some(file_index) => Some(file_index.clone()),
									None => None
								},
							_ => None
						}
					},
					Tag::Egg(egg_tag) => {
						match self.selection {
							Selection::Sprite(index) =>
								match egg_tag.sprites.get(index) {
									Some(file_index) => Some(file_index.clone()),
									None => None
								},
							Selection::Genetics(index) =>
								match egg_tag.genetics.get(index) {
									Some(file_index) => Some(file_index.clone()),
									None => None
								},
							Selection::BodyData(index) =>
								match egg_tag.bodydata.get(index) {
									Some(file_index) => Some(file_index.clone()),
									None => None
								},
							_ => None
						}
					},
					_ => None
				}
			},
			None => None
		}
	}

	pub fn get_selected_file(&self) -> Option<&CreaturesFile> {
		match self.get_selected_file_index() {
			Some(file_index) => self.files.get(file_index),
			None => None
		}
	}

	pub fn get_selected_file_mut(&mut self) -> Option<&mut CreaturesFile> {
		match self.get_selected_file_index() {
			Some(file_index) => self.files.get_mut(file_index),
			None => None
		}
	}
}

#[derive(Clone)]
pub enum Selection {
	None,
	Script(usize),
	Sprite(usize),
	Sound(usize),
	Catalogue(usize),
	Genetics(usize),
	BodyData(usize)
}
