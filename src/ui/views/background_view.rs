use crate::ui::icons::*;
use crate::ui::messages::Message;
use crate::ui::messages::background_message::BackgroundMessage;
use crate::agent::background::*;

use iced::widget::{ row, column, Column, text, button, horizontal_rule };
use iced::{ alignment, Alignment, Length, theme };

pub fn properties(background: &Background) -> Column<Message> {
	match background {
		Background::Blk{ filename } => {
			column![
				row![
					background_icon(),
					text(format!("Background Image \"{}\"", &filename.string)).width(Length::Fill),
					button(delete_icon())
						.on_press(Message::Background(BackgroundMessage::Remove))
						.style(theme::Button::Destructive)
				].spacing(5).align_items(Alignment::Center)
			].padding(30).spacing(20)
		},
		Background::Png{ filename, source } => {
			column![
				row![
					background_icon(),
					text(format!("Background Image \"{}\"", &filename.string)).width(Length::Fill),
					button(delete_icon())
						.on_press(Message::Background(BackgroundMessage::Remove))
						.style(theme::Button::Destructive)
				].spacing(5).align_items(Alignment::Center),
				horizontal_rule(1),
				text(format!("From \"{}\"", &source.string)),
				horizontal_rule(1),
				button(text("Convert to Sprite (.C16)").width(Length::Fill).horizontal_alignment(alignment::Horizontal::Center))
					.style(theme::Button::Secondary)
					.on_press(Message::Background(BackgroundMessage::ConvertToSprite))
					.width(Length::FillPortion(1))
			]
		}
	}
}

pub fn list(backgrounds: &BackgroundList, selected_index: Option<usize>) -> Column<Message> {
	let mut background_list = column![
		row![
			background_icon(),
			text(format!("Background Images ({})", backgrounds.len()))
		].spacing(5),
	].spacing(10);

	for (i, background) in backgrounds.iter().enumerate() {
		let selected = if let Some(index) = selected_index { i == index } else { false };
		let filename = match background {
			Background::Blk{ filename } => filename,
			Background::Png{ filename, .. } => filename
		};
		let mut background_row = row![
			button(filename.string.as_str())
				.on_press(Message::Background(BackgroundMessage::Select(i)))
				.style(if selected { theme::Button::Primary } else { theme::Button::Secondary })
				.width(Length::Fill)
		].spacing(5).align_items(Alignment::Center);
		if backgrounds.len() > 1 {
			background_row = background_row.push(
				button(up_icon())
					.on_press(Message::Background(BackgroundMessage::MoveUp(i)))
					.style(theme::Button::Secondary));
			background_row = background_row.push(
				button(down_icon())
					.on_press(Message::Background(BackgroundMessage::MoveDown(i)))
					.style(theme::Button::Secondary));
		}
		background_list = background_list.push(background_row);
	}

	background_list
}
