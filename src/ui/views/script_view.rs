use crate::ui::messages::Message;
use crate::ui::messages::script_message::ScriptMessage;
use crate::agent::*;
use crate::agent::script::*;

use iced::widget::{ row, column, Column, button, text, radio, horizontal_rule };
use iced::{ Alignment, Length };

pub fn properties(script: &Script) -> Column<Message> {
	match script {
		Script::File{ filename, supported_game } => {
			let supported_game_index = match supported_game {
				SupportedGame::C3DS => Some(0),
				SupportedGame::C3 => Some(1),
				SupportedGame::DS => Some(2)
			};

			column![
				text(format!("Script \"{}\"", &filename.string)),
				horizontal_rule(1),
				row![
					text("Game").width(Length::FillPortion(1)),
					radio("C3 + DS", 0, supported_game_index, |x| Message::Script(ScriptMessage::SetSupportedGame(x)))
						.width(Length::FillPortion(1)),
					radio("C3 only", 1, supported_game_index, |x| Message::Script(ScriptMessage::SetSupportedGame(x)))
						.width(Length::FillPortion(1)),
					radio("DS only", 2, supported_game_index, |x| Message::Script(ScriptMessage::SetSupportedGame(x)))
						.width(Length::FillPortion(1))
				].spacing(5).align_items(Alignment::Center)
			].padding(20).spacing(20)
		}
	}
}

pub fn list(scripts: &ScriptList) -> Column<Message> {
	let mut script_list = column![
		text(format!("Scripts ({})", scripts.len()))
	].spacing(10);

	for (i, script) in scripts.iter().enumerate() {
		let filename = match script {
			Script::File{ filename, .. } => filename
		};
		let buttons = if scripts.len() > 1 {
			row![
				button("^").on_press(Message::Script(ScriptMessage::MoveUp(i))),
				button("v").on_press(Message::Script(ScriptMessage::MoveDown(i))),
				button("x").on_press(Message::Script(ScriptMessage::Remove(i)))
			].spacing(5)
		} else {
			row![
				button("x").on_press(Message::Script(ScriptMessage::Remove(i)))
			].spacing(5)
		};
		script_list = script_list.push(
			row![
				button(filename.string.as_str())
					.on_press(Message::Script(ScriptMessage::Select(i)))
					.width(Length::Fill),
				buttons
			].spacing(5).align_items(Alignment::Center)
		);
	}

	script_list
}
