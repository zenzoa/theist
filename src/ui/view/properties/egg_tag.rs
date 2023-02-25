use crate::ui::Main;
use crate::ui::message::Message;
use crate::ui::message::tag::TagMessage;
use crate::ui::icon::*;
use crate::agent::egg_tag::{ EggTag, EggPreview };

use iced::widget::{ button, checkbox, Column, column, horizontal_rule, horizontal_space, pick_list, row, text, text_input };
use iced::{ Alignment, Length, theme };

pub fn egg_tag_props<'a>(main: &'a Main, egg_tag: &'a EggTag) -> Column<'a, Message> {
	let mut props = column![
		text("Egg Tag Properties:"),

		row![
			text("Name:").width(Length::FillPortion(1)),
			text_input("Name", &egg_tag.name,
				|x| Message::Tag(TagMessage::SetName(x)))
				.width(Length::FillPortion(3))
		].spacing(5).align_items(Alignment::Center),

		row![
			text("Version:").width(Length::FillPortion(1)),
			text_input("Version", &egg_tag.version,
				|x| Message::Tag(TagMessage::SetVersion(x)))
				.width(Length::FillPortion(3))
		].spacing(5).align_items(Alignment::Center),
	];

	// ------
	// Genome
	// ======

	let mut genome_names: Vec<String> = Vec::new();
	for genetics_index in &egg_tag.genetics {
		if let Some(genetics_file) = main.files.get(*genetics_index) {
			if &genetics_file.get_extension() == "gen" {
				genome_names.push(genetics_file.get_title());
			}
		}
	}
	if genome_names.is_empty() {
		genome_names.push("".to_string());
	}

	let current_genome_name = match &egg_tag.genome {
		Some(genome_index) =>
			match main.files.get(*genome_index) {
				Some(genetics_file) => genetics_file.get_title(),
				None => "".to_string()
			},
		None => "".to_string()
	};

	let mut genome = column![
		row![
			text("Genome:").width(Length::FillPortion(1)),
			pick_list(genome_names, Some(current_genome_name.clone()),
				|x| Message::Tag(TagMessage::SetGenome(x)))
				.width(Length::FillPortion(3))
		].spacing(5).align_items(Alignment::Center)
	].spacing(10);

	if current_genome_name.starts_with('e') {
		genome = genome.push(
			column![ text("Note: the Ettin egg-layer may hatch eggs with this genome because it starts with \"e\"") ]
				.padding([ 0, 0, 0, 20 ])
		);
	} else if current_genome_name.starts_with('g') {
		genome = genome.push(
			column![ text("Note: the Grendel egg-layer may hatch eggs with this genome because it starts with \"g\"") ]
				.padding([ 0, 0, 0, 20 ])
		);
	}

	props = props.push(genome);

	// -------
	// Preview
	// =======

	let mut sprite_names: Vec<String> = Vec::new();
	for sprite_index in &egg_tag.sprites {
		if let Some(sprite_file) = main.files.get(*sprite_index) {
			if &sprite_file.get_extension() != "blk" {
				sprite_names.push(sprite_file.get_output_filename());
			}
		}
	}

	let preview_exists = match &egg_tag.preview {
		EggPreview::Manual{ .. } => true,
		EggPreview::None => false
	};

	let mut preview = column![
		row![
			text("Hatchery Preview:").width(Length::FillPortion(1)),
			checkbox("", preview_exists, |x| Message::Tag(TagMessage::SetEggPreviewType(x)))
				.width(Length::FillPortion(3))
		].spacing(5).align_items(Alignment::Center)
	].spacing(10);

	if let EggPreview::Manual{ sprite_male, sprite_female, animation } = &egg_tag.preview {
		let sprite_male_name = match main.files.get(*sprite_male) {
			Some(sprite_file) => sprite_file.get_output_filename(),
			None => "".to_string()
		};
		let sprite_female_name = match main.files.get(*sprite_female) {
			Some(sprite_file) => sprite_file.get_output_filename(),
			None => "".to_string()
		};
		preview = preview.push(
			row![
				text("Male Sprite:").width(Length::FillPortion(1)),
				pick_list(sprite_names.clone(), Some(sprite_male_name),
					|x| Message::Tag(TagMessage::SetEggPreviewSpriteMale(x)))
					.width(Length::FillPortion(3))
			].spacing(5).padding([ 0, 0, 0, 20 ]).align_items(Alignment::Center)
		);
		preview = preview.push(
			row![
				text("Female Sprite:").width(Length::FillPortion(1)),
				pick_list(sprite_names.clone(), Some(sprite_female_name),
					|x| Message::Tag(TagMessage::SetEggPreviewSpriteFemale(x)))
					.width(Length::FillPortion(3))
			].spacing(5).padding([ 0, 0, 0, 20 ]).align_items(Alignment::Center)
		);
		preview = preview.push(
			row![
				text("Animation:").width(Length::FillPortion(1)),
				text_input("Animation String", animation,
					|x| Message::Tag(TagMessage::SetEggPreviewAnimation(x)))
					.width(Length::FillPortion(3))
			].spacing(5).padding([ 0, 0, 0, 20 ]).align_items(Alignment::Center)
		);
	}

	props = props.push(preview);

	// -------------
	// Delete Button
	// =============

	props = props.push(horizontal_rule(1));
	props = props.push(
		button(
				row![
					horizontal_space(Length::Fill),
					delete_icon(),
					text("Delete Tag"),
					horizontal_space(Length::Fill)
				].spacing(5).align_items(Alignment::Center)
			)
			.on_press(Message::Tag(TagMessage::Remove))
			.style(theme::Button::Secondary)
			.width(Length::Fill)
	);

	props
}
