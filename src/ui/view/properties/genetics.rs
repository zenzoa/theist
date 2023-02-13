use crate::ui::Main;
use crate::ui::message::Message;
use crate::ui::message::genetics::GeneticsMessage;
use crate::agent::genetics::Genetics;
use crate::file_helper;

use iced::widget::{ Column, column, row, text, text_input };
use iced::{ Alignment };

pub fn genetics_props<'a>(_main: &'a Main, genetics: &'a Genetics) -> Column<'a, Message> {
	column![
		text("Genetics Properties:"),
		text(format!("Input file: {}", &genetics.input_filename).as_str()),
		row![
			text("Output file:"),
			text_input("Output File Name", &file_helper::title(&genetics.output_filename),
				|x| Message::Genetics(GeneticsMessage::SetName(x))),
			text(format!(".{}", file_helper::extension(&genetics.output_filename)))
		].spacing(5).align_items(Alignment::Center)
	]
}
