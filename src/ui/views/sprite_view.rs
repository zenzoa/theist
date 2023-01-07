use crate::ui::icons::*;
use crate::ui::messages::Message;
use crate::ui::messages::sprite_message::SpriteMessage;
use crate::agent::sprite::*;

use iced::widget::{ row, column, Column, text, text_input, button, scrollable, horizontal_rule };
use iced::{ alignment, Alignment, Length, theme };

pub fn properties(sprite: &Sprite, allow_conversion: bool) -> Column<Message> {
	match sprite {
		Sprite::C16{ filename } => {
			column![
				row![
					sprite_icon(),
					text(format!("Sprite \"{}\"", &filename.string)).width(Length::Fill),
					button(delete_icon())
						.on_press(Message::Sprite(SpriteMessage::Remove))
						.style(theme::Button::Destructive)
				].spacing(5).align_items(Alignment::Center)
			].padding(30).spacing(20)
		},
		Sprite::Frames{ filename, frames } => {
			let mut frame_list = column![
				row![
					text(format!("Frames ({})", frames.len())).width(Length::Fill),
					button(add_icon()).on_press(Message::Sprite(SpriteMessage::AddFrame))
				].spacing(5).align_items(Alignment::Center)
			].spacing(10);

			for (i, frame) in frames.iter().enumerate() {
				let frame_name = frame.filename.to_string();
				if frames.len() > 1 {
					frame_list = frame_list.push(
						row![
							text(frame_name.as_str()).width(Length::Fill),
							button(up_icon())
								.on_press(Message::Sprite(SpriteMessage::MoveFrameUp(i)))
								.style(theme::Button::Secondary),
							button(down_icon())
								.on_press(Message::Sprite(SpriteMessage::MoveFrameDown(i)))
								.style(theme::Button::Secondary),
							button(delete_icon())
								.on_press(Message::Sprite(SpriteMessage::RemoveFrame(i)))
								.style(theme::Button::Destructive)
						].spacing(5).align_items(Alignment::Center)
					);
				} else {
					frame_list = frame_list.push(
						text(frame_name.as_str()).width(Length::Fill)
					);
				}
			}

			if allow_conversion && frames.len() == 1 {
				frame_list = frame_list.push(
					column![
						horizontal_rule(1),
						button(text("Convert to Background Image (.BLK)").width(Length::Fill).horizontal_alignment(alignment::Horizontal::Center))
							.on_press(Message::Sprite(SpriteMessage::ConvertToBackground))
							.style(theme::Button::Secondary)
							.width(Length::FillPortion(1))
					].padding([15, 0, 0, 0]).spacing(20)
				);
			}

			column![
				column![
					row![
						sprite_icon(),
						text(format!("Sprite \"{}\"", &filename.string)).width(Length::Fill),
						button(delete_icon())
							.on_press(Message::Sprite(SpriteMessage::Remove))
							.style(theme::Button::Destructive)
					].spacing(5).align_items(Alignment::Center),
					horizontal_rule(1),
				].padding([30, 30, 0, 30]).spacing(20),
				scrollable(
					column![
						text_input("Name", &filename.title, |x| Message::Sprite(SpriteMessage::SetName(x))),
						frame_list
					].padding(30).spacing(20)
				).height(Length::Fill)
			]
		}
	}
}

pub fn list(sprites: &SpriteList, selected_index: Option<usize>) -> Column<Message> {
	let mut sprite_list = column![
		row![
			sprite_icon(),
			text(format!("Sprites ({})", sprites.len()))
		].spacing(5),
	].spacing(10);

	for (i, sprite) in sprites.iter().enumerate() {
		let selected = if let Some(index) = selected_index { i == index } else { false };
		let filename = match sprite {
			Sprite::C16{ filename } => filename,
			Sprite::Frames{ filename, .. } => filename
		};
		let mut sprite_row = row![
			button(filename.string.as_str())
				.on_press(Message::Sprite(SpriteMessage::Select(i)))
				.style(if selected { theme::Button::Primary } else { theme::Button::Secondary })
				.width(Length::Fill)
		].spacing(5).align_items(Alignment::Center);
		if sprites.len() > 1 {
			sprite_row = sprite_row.push(
				button(up_icon())
					.on_press(Message::Sprite(SpriteMessage::MoveUp(i)))
					.style(theme::Button::Secondary));
			sprite_row = sprite_row.push(
				button(down_icon())
					.on_press(Message::Sprite(SpriteMessage::MoveDown(i)))
					.style(theme::Button::Secondary));
		}
		sprite_list = sprite_list.push(sprite_row);
	}

	sprite_list
}
