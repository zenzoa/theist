use crate::ui::*;
use crate::agent::*;
use crate::agent::egg_tag::*;

use iced::widget::{ row, column, Column, text, text_input, button, radio, checkbox, scrollable, horizontal_rule };
use iced::{ Alignment, Length };

pub fn properties(tag: &EggTag) -> Column<Message> {
	column![
		column![
			row![
				text(format!("Egg Tag \"{}\"", &tag.name)).width(Length::Fill),
				button("x").on_press(Message::DeleteTag)
			].spacing(5).align_items(Alignment::Center),
			horizontal_rule(1),
		].padding([20, 20, 0, 20]).spacing(20),
		scrollable(
			column![
				row![
						text("Name").width(Length::FillPortion(1)),
						text_input("My Egg", &tag.name, Message::SetTagName).width(Length::FillPortion(3))
					].spacing(5).align_items(Alignment::Center),
				row![
						text("Version").width(Length::FillPortion(1)),
						text_input("1.0", &tag.version, Message::SetTagVersion).width(Length::FillPortion(3))
					].spacing(5).align_items(Alignment::Center),
			].padding(20).spacing(20)
		).height(Length::Fill)
	]
}

pub fn listing(tag: &EggTag) -> Column<Message> {
	let mut listing = column![
		row![
			button("+ Add File").on_press(Message::AddFile)
		].spacing(5)
	].spacing(20);

	if !tag.sprites.is_empty() {
		listing = listing.push(sprite_view::list(&tag.sprites));
	}

	listing
}
