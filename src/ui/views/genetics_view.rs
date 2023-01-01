use crate::ui::messages::Message;
// use crate::ui::messages::genetics_message::GeneticsMessage;
use crate::agent::genetics::*;

use iced::widget::{ column, Column, text, horizontal_rule };
// use iced::{ Alignment };

pub fn properties(genetics: &Genetics) -> Column<Message> {
	column![
		// text(format!("Genetics \"{}\"", &genetics.filename.title)),
		// horizontal_rule(1)
	].padding(20).spacing(20)
}

pub fn list(genetics: &GeneticsList) -> Column<Message> {
	let mut genetics_list = column![
		text("Genetics")
	].spacing(10);

	// for (i, genetics) in genetics.iter().enumerate() {
	// 	let buttons = if genetics.len() > 1 {
	// 		row![
	// 			button("^").on_press(Message::Genetics(GeneticsMessage::MoveUp(i))),
	// 			button("v").on_press(Message::Genetics(GeneticsMessage::MoveDown(i))),
	// 			button("x").on_press(Message::Genetics(GeneticsMessage::Remove(i)))
	// 		].spacing(5)
	// 	} else {
	// 		row![
	// 			button("x").on_press(Message::Genetics(GeneticsMessage::Remove(i)))
	// 		].spacing(5)
	// 	};
	// 	genetics_list = genetics_list.push(
	// 		row![
	// 			button(genetics.filename.string.as_str())
	// 				.on_press(Message::Genetics(GeneticsMessage::Select(i)))
	// 				.width(Length::Fill),
	// 			buttons
	// 		].spacing(5).align_items(Alignment::Center)
	// 	);
	// }

	genetics_list
}
