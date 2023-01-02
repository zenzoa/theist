pub mod file_message;
pub mod tag_message;
pub mod script_message;
pub mod sprite_message;
pub mod background_message;
pub mod sound_message;
pub mod catalogue_message;
pub mod genetics_message;
pub mod body_data_message;

use file_message::*;
use tag_message::*;
use script_message::*;
use sprite_message::*;
use background_message::*;
use sound_message::*;
use catalogue_message::*;
use genetics_message::*;
use body_data_message::*;

use crate::ui::{ Main, SelectionType };
use crate::ui::dialogs::*;

use iced::{ Event, window };
use iced::keyboard::KeyCode;
use iced::keyboard::Event::KeyPressed;

#[derive(Debug, Clone)]
pub enum Message {
	EventOccurred(Event),
	File(FileMessage),
	Tag(TagMessage),
	Script(ScriptMessage),
	Sprite(SpriteMessage),
	Background(BackgroundMessage),
	Sound(SoundMessage),
	Catalogue(CatalogueMessage),
	Genetics(GeneticsMessage),
	BodyData(BodyDataMessage)
}

pub fn check_message(main: &mut Main, message: Message) {
	match message {
		Message::EventOccurred(event) => {
			match event {
				Event::Window(window::Event::FileDropped(path)) => {
					let mut result = false;
					if let SelectionType::Sprite(_index) = main.selection_type {
						result = add_sprite_frame_from_path(main, path.clone(), true);
					}
					if !result {
						add_file_from_path(main, path, true);
					}
				},
				Event::Window(window::Event::CloseRequested) => {
					if !main.modified || confirm_exit() {
						main.exit = true;
					}
				},
				Event::Keyboard(KeyPressed{ key_code: KeyCode::Q, .. }) => {
					if !main.modified || confirm_exit() {
						main.exit = true;
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
							save_file_as(main);
						} else {
							save_file(main);
						}
					}
				},
				Event::Keyboard(KeyPressed{ key_code: KeyCode::E, modifiers }) => {
					if modifiers.control() || modifiers.command() {
						println!("compile");
					}
				},
				_ => ()
			}
		},

		Message::File(msg) => check_file_message(main, msg),

		Message::Tag(msg) => check_tag_message(main, msg),

		Message::Script(msg) => check_script_message(main, msg),

		Message::Sprite(msg) => check_sprite_message(main, msg),

		Message::Background(msg) => check_background_message(main, msg),

		Message::Sound(msg) => check_sound_message(main, msg),

		Message::Catalogue(msg) => check_catalogue_message(main, msg),

		Message::Genetics(msg) => check_genetics_message(main, msg),

		Message::BodyData(msg) => check_body_data_message(main, msg)
	}
}
