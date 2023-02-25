use crate::ui::Main;
use crate::ui::message::Message;
use crate::ui::message::tag::TagMessage;
use crate::ui::icon::*;
use crate::agent::agent_tag::{ AgentTag, SupportedGame, Preview, RemoveScript };

use iced::widget::{ button, Column, column, horizontal_rule, horizontal_space, pick_list, radio, row, text, text_input };
use iced::{ Alignment, alignment, Length, theme };

pub fn agent_tag_props<'a>(main: &'a Main, agent_tag: &'a AgentTag) -> Column<'a, Message> {
	let supported_game_index = match agent_tag.supported_game {
		SupportedGame::C3DS => Some(0),
		SupportedGame::C3 => Some(1),
		SupportedGame::DS => Some(2)
	};

	let mut props = column![
		text("Agent Tag Properties:"),

		row![
			text("Name:").width(Length::FillPortion(1)),
			text_input("Name", &agent_tag.name,
				|x| Message::Tag(TagMessage::SetName(x)))
				.width(Length::FillPortion(3))
		].spacing(5).align_items(Alignment::Center),

		row![
			text("Description:").width(Length::FillPortion(1)),
			text_input("Description", &agent_tag.description,
				|x| Message::Tag(TagMessage::SetDescription(x)))
				.width(Length::FillPortion(3))
		].spacing(5).align_items(Alignment::Center),

		row![
			text("Version:").width(Length::FillPortion(1)),
			text_input("Version", &agent_tag.version,
				|x| Message::Tag(TagMessage::SetVersion(x)))
				.width(Length::FillPortion(3))
		].spacing(5).align_items(Alignment::Center),

		row![
			text("Game:").width(Length::FillPortion(1)),
			radio("C3 + DS", 0, supported_game_index,
				|x| Message::Tag(TagMessage::SetSupportedGame(x)))
				.width(Length::FillPortion(1)),
			radio("C3 only", 1, supported_game_index,
				|x| Message::Tag(TagMessage::SetSupportedGame(x)))
				.width(Length::FillPortion(1)),
			radio("DS only", 2, supported_game_index,
				|x| Message::Tag(TagMessage::SetSupportedGame(x)))
				.width(Length::FillPortion(1))
		].spacing(5).align_items(Alignment::Center),
	];

	// -------
	// Preview
	// =======

	let mut sprite_names: Vec<String> = Vec::new();
	for sprite_index in &agent_tag.sprites {
		if let Some(sprite_file) = main.files.get(*sprite_index) {
			if &sprite_file.get_extension() != "blk" {
				sprite_names.push(sprite_file.get_output_filename());
			}
		}
	}

	let preview_index = match &agent_tag.preview {
		Preview::None => Some(0),
		Preview::Auto => Some(1),
		Preview::Manual{ .. } => Some(2)
	};

	let mut preview_options = row![
		text("Injector Preview:").width(Length::FillPortion(1)),
		radio("None", 0, preview_index,
			|x| Message::Tag(TagMessage::SetPreviewType(x)))
			.width(Length::FillPortion(1))
	].spacing(5).align_items(Alignment::Center);

	if sprite_names.is_empty() {
		preview_options = preview_options.push(
			horizontal_space(Length::FillPortion(2))
		);
	} else {
		preview_options = preview_options.push(
			radio("Auto", 1, preview_index,
				|x| Message::Tag(TagMessage::SetPreviewType(x)))
				.width(Length::FillPortion(1)));
		preview_options = preview_options.push(
			radio("Manual", 2, preview_index,
				|x| Message::Tag(TagMessage::SetPreviewType(x)))
				.width(Length::FillPortion(1)));
	}

	let mut preview = column![ preview_options ].spacing(10);

	if let Preview::Manual{ sprite, animation } = &agent_tag.preview {
		let sprite_name = match main.files.get(*sprite) {
			Some(sprite_file) => sprite_file.get_output_filename(),
			None => "".to_string()
		};
		preview = preview.push(
			row![
				pick_list(sprite_names, Some(sprite_name),
					|x| Message::Tag(TagMessage::SetPreviewSprite(x.to_string()))),
				text_input("Animation String", animation,
					|x| Message::Tag(TagMessage::SetPreviewAnimation(x)))
			].spacing(5).align_items(Alignment::Center)
		);
	}

	props = props.push(preview);

	// -------------
	// Remove script
	// =============

	let remove_script_index = match &agent_tag.remove_script {
		RemoveScript::None => Some(0),
		RemoveScript::Auto => Some(1),
		RemoveScript::Manual(_remove_script_string) => Some(2)
	};

	let mut remove_script = column![
		row![
			text("Remove Script:").width(Length::FillPortion(1)),
			radio("None", 0, remove_script_index,
				|x| Message::Tag(TagMessage::SetRemoveScriptType(x)))
				.width(Length::FillPortion(1)),
			radio("Auto", 1, remove_script_index,
				|x| Message::Tag(TagMessage::SetRemoveScriptType(x)))
				.width(Length::FillPortion(1)),
			radio("Manual", 2, remove_script_index,
				|x| Message::Tag(TagMessage::SetRemoveScriptType(x)))
				.width(Length::FillPortion(1))
		].spacing(5).align_items(Alignment::Center)
	].spacing(10);

	if let RemoveScript::Manual(remove_script_string) = &agent_tag.remove_script {
		remove_script = remove_script.push(
			text_input("Remove Script", &remove_script_string, |x| Message::Tag(TagMessage::SetRemoveScript(x)))
		);
	}

	props = props.push(remove_script);

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
