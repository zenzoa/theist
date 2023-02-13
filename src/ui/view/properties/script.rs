use crate::ui::Main;
use crate::ui::message::Message;
use crate::ui::message::script::ScriptMessage;
use crate::agent::script::Script;
use crate::file_helper;

use iced::widget::{ Column, column, row, text, text_input };
use iced::{ Alignment };

pub fn script_props<'a>(_main: &'a Main, script: &'a Script) -> Column<'a, Message> {
	column![
		text("Script Properties:"),
		text(format!("Input file: {}", &script.input_filename).as_str()),
		row![
			text("Output file:"),
			text_input("Output File Name", &file_helper::title(&script.output_filename),
				|x| Message::Script(ScriptMessage::SetName(x))),
			text(".cos")
		].spacing(5).align_items(Alignment::Center)
	]
}
