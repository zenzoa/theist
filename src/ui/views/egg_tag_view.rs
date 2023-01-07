use crate::ui::{ SelectionType };
use crate::ui::icons::*;
use crate::ui::messages::Message;
use crate::ui::messages::tag_message::TagMessage;
use crate::ui::views::{ sprite_view, genetics_view, body_data_view };
use crate::agent::egg_tag::*;
use crate::agent::sprite::*;

use iced::widget::{ row, column, Column, text, text_input, button, pick_list, scrollable, horizontal_rule };
use iced::{ alignment, Alignment, Length, theme };

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
			button(text("Convert to Agent Tag").width(Length::Fill).horizontal_alignment(alignment::Horizontal::Center))
				.on_press(Message::Tag(TagMessage::ConvertToAgent))
				.style(theme::Button::Secondary)
				.width(Length::FillPortion(1))
		);
	}

	column![
		column![
			row![
				tag_icon(),
				text(format!("Egg Tag \"{}\"", &tag.name)).width(Length::Fill),
				button(delete_icon()).on_press(Message::Tag(TagMessage::Remove))
			].spacing(5).align_items(Alignment::Center),
			horizontal_rule(1),
		].padding([30, 30, 0, 30]).spacing(20),
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
			].padding(30).spacing(20)
		).height(Length::Fill)
	]
}

pub fn listing(tag: &EggTag, selection_type: SelectionType) -> Column<Message> {
	let mut listing = column![
		button(row![add_icon(), text("Add File")].spacing(5))
			.on_press(Message::Tag(TagMessage::AddFile))
			.style(theme::Button::Secondary)
	].spacing(20);

	if !tag.genetics.is_empty() {
		let genetics_index = if let SelectionType::Genetics(index) = selection_type { Some(index) } else { None };
		listing = listing.push(genetics_view::list(&tag.genetics, genetics_index));
	}

	if !tag.sprites.is_empty() {
		let sprite_index = if let SelectionType::Sprite(index) = selection_type { Some(index) } else { None };
		listing = listing.push(sprite_view::list(&tag.sprites, sprite_index));
	}

	if !tag.body_data.is_empty() {
		let body_data_index = if let SelectionType::BodyData(index) = selection_type { Some(index) } else { None };
		listing = listing.push(body_data_view::list(&tag.body_data, body_data_index));
	}

	listing
}
