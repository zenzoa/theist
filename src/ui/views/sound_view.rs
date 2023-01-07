use crate::ui::icons::*;
use crate::ui::messages::Message;
use crate::ui::messages::sound_message::SoundMessage;
use crate::agent::sound::*;

use iced::widget::{ row, column, Column, button, text };
use iced::{ Alignment, Length, theme };

pub fn properties(sound: &Sound) -> Column<Message> {
	column![
		row![
			sound_icon(),
			text(format!("Sound \"{}\"", &sound.filename.string)).width(Length::Fill),
			button(delete_icon())
				.on_press(Message::Sound(SoundMessage::Remove))
				.style(theme::Button::Destructive)
		].spacing(5).align_items(Alignment::Center)
	].padding(30).spacing(20)
}

pub fn list(sounds: &SoundList, selected_index: Option<usize>) -> Column<Message> {
	let mut sound_list = column![
		row![
			sound_icon(),
			text(format!("Sounds ({})", sounds.len()))
		].spacing(5),
	].spacing(10);

	for (i, sound) in sounds.iter().enumerate() {
		let selected = if let Some(index) = selected_index { i == index } else { false };
		let mut sound_row = row![
			button(sound.filename.string.as_str())
				.on_press(Message::Sound(SoundMessage::Select(i)))
				.style(if selected { theme::Button::Primary } else { theme::Button::Secondary })
				.width(Length::Fill)
		].spacing(5).align_items(Alignment::Center);
		if sounds.len() > 1 {
			sound_row = sound_row.push(
				button(up_icon())
					.on_press(Message::Sound(SoundMessage::MoveUp(i)))
					.style(theme::Button::Secondary));
			sound_row = sound_row.push(
				button(down_icon())
					.on_press(Message::Sound(SoundMessage::MoveDown(i)))
					.style(theme::Button::Secondary));
		}
		sound_list = sound_list.push(sound_row);
	}

	sound_list
}
