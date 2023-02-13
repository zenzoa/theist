use crate::ui::Main;
use crate::ui::message::Message;
use crate::ui::message::bodydata::BodyDataMessage;
use crate::agent::bodydata::BodyData;
use crate::file_helper;

use iced::widget::{ Column, column, row, text, text_input };
use iced::{ Alignment };

pub fn bodydata_props<'a>(_main: &'a Main, bodydata: &'a BodyData) -> Column<'a, Message> {
	column![
		text("Body Data Properties:"),
		text(format!("Input file: {}", &bodydata.input_filename).as_str()),
		row![
			text("Output file:"),
			text_input("Output File Name", &file_helper::title(&bodydata.output_filename),
				|x| Message::BodyData(BodyDataMessage::SetName(x))),
			text(".att")
		].spacing(5).align_items(Alignment::Center)
	]
}
