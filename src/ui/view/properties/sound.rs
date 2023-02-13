use crate::ui::Main;
use crate::ui::message::Message;
use crate::ui::message::sound::SoundMessage;
use crate::agent::sound::Sound;
use crate::file_helper;

use iced::widget::{ Column, column, row, text, text_input };
use iced::{ Alignment };

pub fn sound_props<'a>(_main: &'a Main, sound: &'a Sound) -> Column<'a, Message> {
	column![
		text("Sound Properties:"),
		text(format!("Input file: {}", &sound.input_filename).as_str()),
		row![
			text("Output file:"),
			text_input("Output File Name", &file_helper::title(&sound.output_filename),
				|x| Message::Sound(SoundMessage::SetName(x))),
			text(".wav")
		].spacing(5).align_items(Alignment::Center)
	]
}

