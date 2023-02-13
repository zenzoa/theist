use crate::ui::Main;
use crate::ui::message::Message;
use crate::ui::message::sprite::SpriteMessage;
use crate::ui::icon::*;
use crate::agent::sprite::Sprite;
use crate::file_helper;

use iced::widget::{ button, Column, column, pick_list, row, text, text_input };
use iced::{ Alignment, Length, theme };

pub fn sprite_props<'a>(_main: &'a Main, sprite: &'a Sprite) -> Column<'a, Message> {
	match sprite {
		Sprite::Raw{ input_filename, output_filename, .. } =>
			column![
				text("Sprite Properties:"),
				text(format!("Input file: {}", input_filename).as_str()),
				row![
					text("Output file:"),
					text_input("Output File Name", &file_helper::title(output_filename),
						|x| Message::Sprite(SpriteMessage::SetName(x))),
					text(&format!(".{}", file_helper::extension(output_filename)))
				].spacing(5).align_items(Alignment::Center)
			],

		Sprite::Png{ input_filename, output_filename, frames, .. } => {
			let mut props = column![ text("Sprite Properties:") ];

			let extension = file_helper::extension(output_filename);
			let mut extension_list = vec![ "c16".to_string(), "s16".to_string() ];
			if frames.len() < 2 {
				extension_list.push("blk".to_string());
			}

			if frames.len() == 1 {
				props = props.push(text(format!("Input file: {}", input_filename).as_str()));
			} else {
				props = props.push(text("Input file: (multiple)"));
			}

			props = props.push(row![
					text("Output file:"),
					text_input("Output File Name", &file_helper::title(output_filename),
						|x| Message::Sprite(SpriteMessage::SetName(x))),
					pick_list(extension_list, Some(extension.to_string()), |x| Message::Sprite(SpriteMessage::SetExtension(x)))
				].spacing(5).align_items(Alignment::Center));

			if extension == "c16" || extension == "s16" {
				let mut frame_list = column![
					row![
						text("Frames:").width(Length::Fill),
						button(add_icon())
							.on_press(Message::Sprite(SpriteMessage::AddFrame))
							.style(theme::Button::Secondary)
					].align_items(Alignment::Center)
				].spacing(10);

				for (i, frame) in frames.iter().enumerate() {
					let mut frame_row = row![
						text(frame.input_filename.as_str()).width(Length::Fill)
					].spacing(5).align_items(Alignment::Center);

					if frames.len() > 1 {
						frame_row = frame_row.push(
							button(up_icon())
								.on_press(Message::Sprite(SpriteMessage::MoveFrameUp(i)))
								.style(theme::Button::Secondary));
						frame_row = frame_row.push(
							button(down_icon())
								.on_press(Message::Sprite(SpriteMessage::MoveFrameDown(i)))
								.style(theme::Button::Secondary));
						frame_row = frame_row.push(
							button(delete_icon())
								.on_press(Message::Sprite(SpriteMessage::RemoveFrame(i)))
								.style(theme::Button::Secondary));
					}

					frame_list = frame_list.push(frame_row);
				}

				props = props.push(frame_list);
			}

			props
		}
	}
}
