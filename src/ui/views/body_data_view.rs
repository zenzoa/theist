use crate::ui::messages::Message;
use crate::ui::messages::body_data_message::BodyDataMessage;
use crate::agent::body_data::*;

use iced::widget::{ row, column, Column, text, button, horizontal_rule };
use iced::{ Alignment, Length };

pub fn properties(body_data: &BodyData) -> Column<Message> {
	column![
		text(format!("Body Data \"{}\"", &body_data.filename.string)),
		horizontal_rule(1)
	].padding(20).spacing(20)
}

pub fn list(body_data: &BodyDataList) -> Column<Message> {
	let mut body_data_list = column![
		text(format!("Body Data ({})", body_data.len()))
	].spacing(10);

	for (i, body_data_file) in body_data.iter().enumerate() {
		let buttons = if body_data.len() > 1 {
			row![
				button("^").on_press(Message::BodyData(BodyDataMessage::MoveUp(i))),
				button("v").on_press(Message::BodyData(BodyDataMessage::MoveDown(i))),
				button("x").on_press(Message::BodyData(BodyDataMessage::Remove(i)))
			].spacing(5)
		} else {
			row![
				button("x").on_press(Message::BodyData(BodyDataMessage::Remove(i)))
			].spacing(5)
		};
		body_data_list = body_data_list.push(
			row![
				button(body_data_file.filename.string.as_str())
					.on_press(Message::BodyData(BodyDataMessage::Select(i)))
					.width(Length::Fill),
				buttons
			].spacing(5).align_items(Alignment::Center)
		);
	}

	body_data_list
}
