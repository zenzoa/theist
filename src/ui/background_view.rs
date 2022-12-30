use crate::ui::*;
use crate::agent::background::*;

use iced::widget::{ row, column, Column, text, button, horizontal_rule };
use iced::{ Alignment, Length };

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

pub fn list(backgrounds: &Vec<Background>) -> Column<Message> {
	let mut background_list = column![
		text("Background Images")
	].spacing(10);

	for (i, background) in backgrounds.iter().enumerate() {
		let filename = match background {
			Background::Blk{ filename } => filename,
			Background::Png{ filename, .. } => filename
		};
		let buttons = if backgrounds.len() > 1 {
			row![
				button("^").on_press(Message::MoveBackgroundUp(i)),
				button("v").on_press(Message::MoveBackgroundDown(i)),
				button("x").on_press(Message::DeleteBackground(i))
			].spacing(5)
		} else {
			row![
				button("x").on_press(Message::DeleteBackground(i))
			].spacing(5)
		};
		background_list = background_list.push(
			row![
				button(filename.string.as_str())
					.on_press(Message::SelectBackground(i))
					.width(Length::Fill),
				buttons
			].spacing(5).align_items(Alignment::Center)
		);
	}

	background_list
}
