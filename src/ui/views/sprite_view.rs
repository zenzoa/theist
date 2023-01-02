use crate::ui::messages::Message;
use crate::ui::messages::sprite_message::SpriteMessage;
use crate::agent::sprite::*;

use iced::widget::{ row, column, Column, text, text_input, button, scrollable, horizontal_rule };
use iced::{ Alignment, Length };

pub fn properties(sprite: &Sprite, allow_conversion: bool) -> Column<Message> {
	match sprite {
		Sprite::C16{ filename } => {
			column![
				text(format!("Sprite \"{}\"", &filename.string)),
				horizontal_rule(1)
			].padding(20).spacing(20)
		},
		Sprite::Frames{ filename, frames } => {
			let mut frame_list = column![
				row![
					text(format!("Frames ({})", frames.len())).width(Length::Fill),
					button("+").on_press(Message::Sprite(SpriteMessage::AddFrame))
				].spacing(5).align_items(Alignment::Center)
			].spacing(10);

			for (i, frame) in frames.iter().enumerate() {
				let frame_name = frame.filename.to_string();
				if frames.len() > 1 {
					frame_list = frame_list.push(
						row![
							text(frame_name.as_str()).width(Length::Fill),
							button("^").on_press(Message::Sprite(SpriteMessage::MoveFrameUp(i))),
							button("v").on_press(Message::Sprite(SpriteMessage::MoveFrameDown(i))),
							button("x").on_press(Message::Sprite(SpriteMessage::RemoveFrame(i)))
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
						button("Convert to Background Image (.BLK)")
							.on_press(Message::Sprite(SpriteMessage::ConvertToBackground))
							.width(Length::FillPortion(1))
					].padding([15, 0, 0, 0]).spacing(20)
				);
			}

			column![
				column![
					text(format!("Sprite \"{}\"", &filename.string)),
					horizontal_rule(1),
				].padding([20, 20, 0, 20]).spacing(20),
				scrollable(
					column![
						text_input("Name", &filename.title, |x| Message::Sprite(SpriteMessage::SetName(x))),
						frame_list
					].padding(20).spacing(20)
				).height(Length::Fill)
			]
		}
	}
}

pub fn list(sprites: &SpriteList) -> Column<Message> {
	let mut sprite_list = column![
		text(format!("Sprites ({})", sprites.len()))
	].spacing(10);

	for (i, sprite) in sprites.iter().enumerate() {
		let filename = match sprite {
			Sprite::C16{ filename } => filename,
			Sprite::Frames{ filename, .. } => filename
		};
		let buttons = if sprites.len() > 1 {
			row![
				button("^").on_press(Message::Sprite(SpriteMessage::MoveUp(i))),
				button("v").on_press(Message::Sprite(SpriteMessage::MoveDown(i))),
				button("x").on_press(Message::Sprite(SpriteMessage::Remove(i)))
			].spacing(5)
		} else {
			row![
				button("x").on_press(Message::Sprite(SpriteMessage::Remove(i)))
			].spacing(5)
		};
		sprite_list = sprite_list.push(
			row![
				button(filename.string.as_str())
					.on_press(Message::Sprite(SpriteMessage::Select(i)))
					.width(Length::Fill),
				buttons
			].spacing(5).align_items(Alignment::Center)
		);
	}

	sprite_list
}
