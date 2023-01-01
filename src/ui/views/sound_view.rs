use crate::ui::messages::Message;
use crate::ui::messages::sound_message::SoundMessage;
use crate::agent::sound::*;

use iced::widget::{ row, column, Column, button, text, horizontal_rule };
use iced::{ Alignment, Length };

pub fn properties(sound: &Sound) -> Column<Message> {
	column![
		text(format!("Sound \"{}\"", &sound.filename.title)),
		horizontal_rule(1)
	].padding(20).spacing(20)
}

pub fn list(sounds: &SoundList) -> Column<Message> {
	let mut sound_list = column![
		text("Sounds")
	].spacing(10);

	for (i, sound) in sounds.iter().enumerate() {
		let buttons = if sounds.len() > 1 {
			row![
				button("^").on_press(Message::Sound(SoundMessage::MoveUp(i))),
				button("v").on_press(Message::Sound(SoundMessage::MoveDown(i))),
				button("x").on_press(Message::Sound(SoundMessage::Remove(i)))
			].spacing(5)
		} else {
			row![
				button("x").on_press(Message::Sound(SoundMessage::Remove(i)))
			].spacing(5)
		};
		sound_list = sound_list.push(
			row![
				button(sound.filename.string.as_str())
					.on_press(Message::Sound(SoundMessage::Select(i)))
					.width(Length::Fill),
				buttons
			].spacing(5).align_items(Alignment::Center)
		);
	}

	sound_list
}
