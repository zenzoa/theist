use crate::ui::*;
use crate::agent::script::*;

use iced::widget::{ row, column, Column, text, radio };
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
				text(format!("Script \"{}\"", &filename.title)),
				horizontal_rule(1),
				row![
					text("Game").width(Length::FillPortion(1)),
					radio("C3 + DS", 0, supported_game_index, Message::SetScriptSupportedGame).width(Length::FillPortion(1)),
					radio("C3 only", 1, supported_game_index, Message::SetScriptSupportedGame).width(Length::FillPortion(1)),
					radio("DS only", 2, supported_game_index, Message::SetScriptSupportedGame).width(Length::FillPortion(1))
				].spacing(5).align_items(Alignment::Center)
			].padding(20).spacing(20)
		}
	}
}
