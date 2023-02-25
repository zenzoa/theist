pub mod file;
pub mod tag;
pub mod script;
pub mod sprite;
pub mod sound;
pub mod catalogue;
pub mod genetics;
pub mod bodydata;

use file::*;
use tag::{ TagMessage, check_tag_message, add_file_from_path };
use script::{ ScriptMessage, check_script_message };
use sprite::{ SpriteMessage, check_sprite_message };
use sound::{ SoundMessage, check_sound_message };
use catalogue::{ CatalogueMessage, check_catalogue_message };
use genetics::{ GeneticsMessage, check_genetics_message };
use bodydata::{ BodyDataMessage, check_bodydata_message };
use super::Main;
use super::dialog::*;

use iced::{ Command, Event, window };
use iced::keyboard::KeyCode;
use iced::keyboard::Event::KeyPressed;

#[derive(Debug, Clone)]
pub enum Message {
	EventOccurred(Event),
	DismissAlert(usize),
	File(FileMessage),

	ShowNewTagDialog,
	HideNewTagDialog,
	ShowExistingFileDialog,
	HideExistingFileDialog,

	Tag(TagMessage),
	Script(ScriptMessage),
	Sprite(SpriteMessage),
	Sound(SoundMessage),
	Catalogue(CatalogueMessage),
	Genetics(GeneticsMessage),
	BodyData(BodyDataMessage)
}

pub fn check_message(main: &mut Main, message: Message) -> Command<Message> {
	match message {
		Message::EventOccurred(event) => {
			match event {
				Event::Window(window::Event::FileDropped(path)) => {
					let filepath = path.to_string_lossy().into_owned();
					if (filepath.ends_with(".the") ||
						filepath.ends_with(".txt") ||
						filepath.ends_with(".agent") ||
						filepath.ends_with(".agents")) &&
						(!main.modified || confirm_discard_changes())
					{
						open_file_from_path(main, filepath);
					} else {
						add_file_from_path(main, filepath, true);
					}
				},

				Event::Window(window::Event::CloseRequested) => {
					if !main.modified || confirm_exit() {
						return window::close();
					}
				},

				Event::Keyboard(KeyPressed{ key_code: KeyCode::Q, .. }) => {
					if !main.modified || confirm_exit() {
						return window::close();
					}
				},

				Event::Keyboard(KeyPressed{ key_code: KeyCode::N, modifiers }) => {
					if modifiers.control() || modifiers.command() {
						new_file(main);
					}
				},

				Event::Keyboard(KeyPressed{ key_code: KeyCode::O, modifiers }) => {
					if modifiers.control() || modifiers.command() {
						open_file(main);
					}
				},

				Event::Keyboard(KeyPressed{ key_code: KeyCode::S, modifiers }) => {
					if modifiers.control() || modifiers.command() {
						if modifiers.shift() {
							save_file_as(main, main.filename.clone());
						} else {
							save_file(main);
						}
					}
				},

				Event::Keyboard(KeyPressed{ key_code: KeyCode::E, modifiers }) => {
					if modifiers.control() || modifiers.command() {
						compile(main);
					}
				},

				_ => ()
			}
		},

		Message::DismissAlert(index) => {
			main.alerts.remove(index);
		},

		Message::ShowNewTagDialog => {
			main.is_adding_new_tag = true;
		},

		Message::HideNewTagDialog => {
			main.is_adding_new_tag = false;
		},

		Message::ShowExistingFileDialog => {
			main.is_adding_existing_file = true;
		},

		Message::HideExistingFileDialog => {
			main.is_adding_existing_file = false;
		},

		Message::File(msg) => check_file_message(main, msg),

		Message::Tag(msg) => check_tag_message(main, msg),

		Message::Script(msg) => check_script_message(main, msg),

		Message::Sprite(msg) => check_sprite_message(main, msg),

		Message::Sound(msg) => check_sound_message(main, msg),

		Message::Catalogue(msg) => check_catalogue_message(main, msg),

		Message::Genetics(msg) => check_genetics_message(main, msg),

		Message::BodyData(msg) => check_bodydata_message(main, msg)
	}

	Command::none()
}
