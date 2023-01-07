use crate::ui::icons::*;
use crate::ui::messages::Message;
use crate::ui::messages::genetics_message::GeneticsMessage;
use crate::agent::genetics::*;

use iced::widget::{ row, column, Column, text, button };
use iced::{ Alignment, Length, theme };

pub fn properties(genetics: &Genetics) -> Column<Message> {
	column![
		row![
			genetics_icon(),
			text(format!("Genetics \"{}\"", &genetics.filename.string)).width(Length::Fill),
			button(delete_icon())
				.on_press(Message::Genetics(GeneticsMessage::Remove))
				.style(theme::Button::Destructive)
		].spacing(5).align_items(Alignment::Center)
	].padding(30).spacing(20)
}

pub fn list(genetics: &GeneticsList, selected_index: Option<usize>) -> Column<Message> {
	let mut genetics_list = column![
		row![
			genetics_icon(),
			text(format!("Genetics ({})", genetics.len()))
		].spacing(5),
	].spacing(10);

	for (i, genetics_file) in genetics.iter().enumerate() {
		let selected = if let Some(index) = selected_index { i == index } else { false };
		let mut genetics_row = row![
			button(genetics_file.filename.string.as_str())
				.on_press(Message::Genetics(GeneticsMessage::Select(i)))
				.style(if selected { theme::Button::Primary } else { theme::Button::Secondary })
				.width(Length::Fill)
		].spacing(5).align_items(Alignment::Center);
		if genetics.len() > 1 {
			genetics_row = genetics_row.push(
				button(up_icon())
					.on_press(Message::Genetics(GeneticsMessage::MoveUp(i)))
					.style(theme::Button::Secondary));
			genetics_row = genetics_row.push(
				button(down_icon())
					.on_press(Message::Genetics(GeneticsMessage::MoveDown(i)))
					.style(theme::Button::Secondary));
		}
		genetics_list = genetics_list.push(genetics_row);
	}

	genetics_list
}
