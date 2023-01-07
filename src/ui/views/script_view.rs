use crate::ui::icons::*;
use crate::ui::messages::Message;
use crate::ui::messages::script_message::ScriptMessage;
use crate::agent::*;
use crate::agent::script::*;

use iced::widget::{ row, column, Column, button, text, radio, horizontal_rule };
use iced::{ Alignment, Length, theme };

pub fn properties(script: &Script) -> Column<Message> {
	match script {
		Script::File{ filename, supported_game } => {
			let supported_game_index = match supported_game {
				SupportedGame::C3DS => Some(0),
				SupportedGame::C3 => Some(1),
				SupportedGame::DS => Some(2)
			};

			column![
				row![
					script_icon(),
					text(format!("Script \"{}\"", &filename.string)).width(Length::Fill),
					button(delete_icon())
						.on_press(Message::Script(ScriptMessage::Remove))
						.style(theme::Button::Destructive)
				].spacing(5).align_items(Alignment::Center),
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
			].padding(30).spacing(20)
		}
	}
}

pub fn list(scripts: &ScriptList, selected_index: Option<usize>) -> Column<Message> {
	let mut script_list = column![
		row![
			script_icon(),
			text(format!("Scripts ({})", scripts.len()))
		].spacing(5),
	].spacing(10);

	for (i, script) in scripts.iter().enumerate() {
		let selected = if let Some(index) = selected_index { i == index } else { false };
		let filename = match script {
			Script::File{ filename, .. } => filename
		};
		let mut script_row = row![
			button(filename.string.as_str())
				.on_press(Message::Script(ScriptMessage::Select(i)))
				.style(if selected { theme::Button::Primary } else { theme::Button::Secondary })
				.width(Length::Fill)
		].spacing(5).align_items(Alignment::Center);
		if scripts.len() > 1 {
			script_row = script_row.push(
				button(up_icon())
					.on_press(Message::Script(ScriptMessage::MoveUp(i)))
					.style(theme::Button::Secondary));
			script_row = script_row.push(
				button(down_icon())
					.on_press(Message::Script(ScriptMessage::MoveDown(i)))
					.style(theme::Button::Secondary));
		}
		script_list = script_list.push(script_row);
	}

	script_list
}
