use crate::ui::icons::*;
use crate::ui::messages::Message;
use crate::ui::messages::body_data_message::BodyDataMessage;
use crate::agent::body_data::*;

use iced::widget::{ row, column, Column, text, button };
use iced::{ Alignment, Length, theme };

pub fn properties(body_data: &BodyData) -> Column<Message> {
	column![
		row![
			body_data_icon(),
			text(format!("Body Data \"{}\"", &body_data.filename.string)).width(Length::Fill),
			button(delete_icon())
				.on_press(Message::BodyData(BodyDataMessage::Remove))
				.style(theme::Button::Destructive)
		].spacing(5).align_items(Alignment::Center)
	].padding(30).spacing(20)
}

pub fn list(body_data: &BodyDataList, selected_index: Option<usize>) -> Column<Message> {
	let mut body_data_list = column![
		row![
			body_data_icon(),
			text(format!("Body Data ({})", body_data.len()))
		].spacing(5),
	].spacing(10);

	for (i, body_data_file) in body_data.iter().enumerate() {
		let selected = if let Some(index) = selected_index { i == index } else { false };
		let mut body_data_row = row![
			button(body_data_file.filename.string.as_str())
				.on_press(Message::BodyData(BodyDataMessage::Select(i)))
				.style(if selected { theme::Button::Primary } else { theme::Button::Secondary })
				.width(Length::Fill)
		].spacing(5).align_items(Alignment::Center);
		if body_data.len() > 1 {
			body_data_row = body_data_row.push(
				button(up_icon())
					.on_press(Message::BodyData(BodyDataMessage::MoveUp(i)))
					.style(theme::Button::Secondary));
			body_data_row = body_data_row.push(
				button(down_icon())
					.on_press(Message::BodyData(BodyDataMessage::MoveDown(i)))
					.style(theme::Button::Secondary));
		}
		body_data_list = body_data_list.push(body_data_row);
	}

	body_data_list
}
