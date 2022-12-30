use crate::ui::*;
use crate::agent::background::*;

use iced::widget::{ column, Column, text, button, horizontal_rule };

pub fn properties(background: &Background) -> Column<Message> {
	match background {
		Background::Blk{ filename } => {
			column![
				text(format!("Background Image \"{}\"", &filename.title)),
				horizontal_rule(1)
			].padding(20).spacing(20)
		},
		Background::Png{ filename, source } => {
			column![
				text(format!("Background Image \"{}\"", &filename.title)),
				horizontal_rule(1),
				text(format!("From \"{}\"", &source.string)),
				horizontal_rule(1),
				button("Convert to Sprite (.C16)")
					.on_press(Message::ConvertBackgroundToSprite)
					.width(Length::FillPortion(1))
			].padding(20).spacing(20)
		}
	}
}
