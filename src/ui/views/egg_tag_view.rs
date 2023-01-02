use crate::ui::messages::Message;
use crate::ui::messages::tag_message::TagMessage;
use crate::ui::views::{ sprite_view, genetics_view, body_data_view };
use crate::agent::egg_tag::*;
use crate::agent::sprite::*;

use iced::widget::{ row, column, Column, text, text_input, button, pick_list, scrollable, horizontal_rule };
use iced::{ Alignment, Length };

pub fn properties(tag: &EggTag) -> Column<Message> {
	let sprite_names: Vec<String> = tag.sprites.iter().map(|sprite| {
		match sprite {
			Sprite::C16{ filename, .. } => filename.title.clone(),
			Sprite::Frames{ filename, .. } => filename.title.clone()
		}
	}).collect();

	let mut convert_to_agent = column![].spacing(20);

	if tag.genetics.is_empty() && tag.body_data.is_empty() {
		convert_to_agent = convert_to_agent.push(horizontal_rule(1));
		convert_to_agent = convert_to_agent.push(
			button("Convert to Agent Tag")
				.on_press(Message::Tag(TagMessage::ConvertToAgent))
				.width(Length::FillPortion(1))
		);
	}

	column![
		column![
			row![
				text(format!("Egg Tag \"{}\"", &tag.name)).width(Length::Fill),
				button("x").on_press(Message::Tag(TagMessage::Remove))
			].spacing(5).align_items(Alignment::Center),
			horizontal_rule(1),
		].padding([20, 20, 0, 20]).spacing(20),
		scrollable(
			column![
				row![
						text("Name").width(Length::FillPortion(1)),
						text_input("My Egg", &tag.name, |x| Message::Tag(TagMessage::SetName(x)))
							.width(Length::FillPortion(3))
					].spacing(5).align_items(Alignment::Center),
				row![
						text("Version").width(Length::FillPortion(1)),
						text_input("1.0", &tag.version, |x| Message::Tag(TagMessage::SetVersion(x)))
							.width(Length::FillPortion(3))
					].spacing(5).align_items(Alignment::Center),
				row![
						text("Female Sprite").width(Length::FillPortion(1)),
						pick_list(sprite_names.clone(), Some(tag.preview_sprite_female.to_string()), |x| Message::Tag(TagMessage::SetFemalePreviewSprite(x)))
							.width(Length::FillPortion(3))
					].spacing(5).align_items(Alignment::Center),
				row![
						text("Male Sprite").width(Length::FillPortion(1)),
						pick_list(sprite_names, Some(tag.preview_sprite_male.to_string()), |x| Message::Tag(TagMessage::SetMalePreviewSprite(x)))
							.width(Length::FillPortion(3))
					].spacing(5).align_items(Alignment::Center),
				row![
						text("Animation").width(Length::FillPortion(1)),
						text_input("0", &tag.preview_animation, |x| Message::Tag(TagMessage::SetPreviewAnimation(x)))
							.width(Length::FillPortion(3))
					].spacing(5).align_items(Alignment::Center),
				convert_to_agent
			].padding(20).spacing(20)
		).height(Length::Fill)
	]
}

pub fn listing(tag: &EggTag) -> Column<Message> {
	let mut listing = column![
		row![
			button("+ Add File").on_press(Message::Tag(TagMessage::AddFile))
		].spacing(5)
	].spacing(20);

	if !tag.genetics.is_empty() {
		listing = listing.push(genetics_view::list(&tag.genetics));
	}

	if !tag.sprites.is_empty() {
		listing = listing.push(sprite_view::list(&tag.sprites));
	}

	if !tag.body_data.is_empty() {
		listing = listing.push(body_data_view::list(&tag.body_data));
	}

	listing
}
