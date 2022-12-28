use crate::ui::*;
use crate::agent::sound::*;

use iced::widget::{ column, Column, text, horizontal_rule };

pub fn properties(sound: &Sound) -> Column<Message> {
	column![
		text(format!("Sound \"{}\"", &sound.filename.title)),
		horizontal_rule(1)
	].padding(20).spacing(20)
}
