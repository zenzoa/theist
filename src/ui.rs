pub mod icons;
pub mod messages;
pub mod dialogs;
pub mod views;

use icons::*;
use messages::{ Message, check_message };
use messages::file_message::FileMessage;
use messages::tag_message::TagMessage;
use views::*;

use crate::agent::*;
use crate::agent::tag::*;
use crate::agent::agent_tag::*;

use std::str;
use std::path::Path;
use iced::widget::{ row, column, container, text, button, scrollable, horizontal_space, horizontal_rule };
use iced::{ alignment, Alignment, Application, Command, Element, executor, Length, subscription, Subscription, theme, Theme };

#[derive(Clone)]
pub enum SelectionType {
	Tag,
	Script(usize),
	Sprite(usize),
	Background(usize),
	Sound(usize),
	Catalogue(usize),
	Genetics(usize),
	BodyData(usize)
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
			button(row![new_icon(), text("New")].spacing(5))
				.on_press(Message::File(FileMessage::New))
				.style(theme::Button::Secondary),
			button(row![open_icon(), text("Open")].spacing(5))
				.on_press(Message::File(FileMessage::Open))
				.style(theme::Button::Secondary),
			button(row![save_icon(), text("Save")].spacing(5))
				.on_press(Message::File(FileMessage::Save))
				.style(theme::Button::Secondary),
			button("Save As")
				.on_press(Message::File(FileMessage::SaveAs))
				.style(theme::Button::Secondary),
			horizontal_space(Length::Fill),
			button(row![compile_icon(), text("Compile")].spacing(5))
				.on_press(Message::File(FileMessage::Compile))
				.style(theme::Button::Secondary)
		].padding(10).spacing(5);

		let mut tabs = row![].spacing(5).align_items(Alignment::Center);

		for (i, tag) in self.tags.iter().enumerate() {
			let selection_is_tag = matches!(self.selection_type, SelectionType::Tag);
			let selected = if let Some(index) = self.selected_tag { i == index } else { false };
			let tag_name = match tag {
				Tag::Agent(agent_tag) => &agent_tag.name,
				Tag::Egg(egg_tag) => &egg_tag.name,
				Tag::Empty => ""
			};
			tabs = tabs.push(
				button(
					button(
						text(tag_name)
							.horizontal_alignment(alignment::Horizontal::Center)
							.width(Length::Fill)
					)
					.on_press(Message::Tag(TagMessage::Select(Some(i))))
					.style(if selected && selection_is_tag { theme::Button::Primary } else { theme::Button::Secondary })
					.width(Length::Fill)
				)
				.on_press(Message::Tag(TagMessage::Select(Some(i))))
				.padding(2)
				.style(if selected { theme::Button::Primary } else { theme::Button::Text })
				.width(Length::FillPortion(1))
			);
		}

		tabs = tabs.push(button(add_icon())
			.on_press(Message::Tag(TagMessage::Add)));

		let mut tab_contents = column![];
		let mut current_properties = column![];

		if let Some(selected_tag) = self.selected_tag {
			if let Some(tag) = self.tags.get(selected_tag) {
				match tag {
					Tag::Agent(tag) => {
						tab_contents = agent_tag_view::listing(tag, self.selection_type.clone());
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
						tab_contents = egg_tag_view::listing(tag, self.selection_type.clone());
						match self.selection_type {
							SelectionType::Genetics(index) => {
								if let Some(genetics) = tag.genetics.get(index) {
									current_properties = genetics_view::properties(genetics);
								}
							},
							SelectionType::Sprite(index) => {
								if let Some(sprite) = tag.sprites.get(index) {
									current_properties = sprite_view::properties(sprite, false);
								}
							},
							SelectionType::BodyData(index) => {
								if let Some(body_data) = tag.body_data.get(index) {
									current_properties = body_data_view::properties(body_data);
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
			tabs.padding([30, 30, 10, 28]),
			scrollable(
				tab_contents.padding([0, 30, 30, 30])
			).height(Length::Fill)
		];

		let mut alerts_container = column![];

		if !self.alerts.is_empty() {
			let mut alerts_pane = column![].padding(10).spacing(5);

			for (i, alert) in self.alerts.iter().enumerate() {
				match alert {
					Alert::Update(message) => {
						alerts_pane = alerts_pane.push(
							button(
								row![
									alert_icon(),
									text(&message).width(Length::Fill),
									dismiss_icon()
								].spacing(5))
								.on_press(Message::DismissAlert(i))
								.style(theme::Button::Positive)
						)
					},
					Alert::Error(message) => {
						alerts_pane = alerts_pane.push(
							button(
								row![
									error_icon(),
									text(&message).width(Length::Fill),
									dismiss_icon()
								].spacing(5))
								.on_press(Message::DismissAlert(i))
								.style(theme::Button::Destructive)
						)
					}
				}
			}

			alerts_container = column![
				horizontal_rule(1),
				alerts_pane
			]
		}

		column![
			toolbar,
			horizontal_rule(1),
			row![
					main_pane.width(Length::FillPortion(1)),
					container(current_properties)
						.style(theme::Container::Box)
						.width(Length::FillPortion(1))
						.height(Length::Fill)
				].height(Length::Fill),
			alerts_container
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

	fn clear_alerts(&mut self) {
		self.alerts.clear();
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
