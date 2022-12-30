use crate::ui::*;
use crate::agent::*;
use crate::agent::agent_tag::*;

use iced::widget::{ row, column, Column, text, text_input, button, radio, checkbox, pick_list, scrollable, horizontal_rule };
use iced::{ Alignment, Length };

pub fn properties(tag: &AgentTag) -> Column<Message> {
	let supported_game_index = match tag.supported_game {
		SupportedGame::C3DS => Some(0),
		SupportedGame::C3 => Some(1),
		SupportedGame::DS => Some(2)
	};

	let mut preview = column![
		row![
			text("Injector Preview"),
			checkbox("Auto", tag.preview == Preview::Auto, Message::SetTagPreviewAuto)
		].spacing(20).align_items(Alignment::Center)
	].spacing(10);

	let sprite_names: Vec<String> = tag.sprites.iter().map(|sprite| {
		match sprite {
			Sprite::C16{ filename, .. } => filename.title.clone(),
			Sprite::Frames{ filename, .. } => filename.title.clone()
		}
	}).collect();

	if let Preview::Manual { sprite, animation } = &tag.preview {
		preview = preview.push(
			row![
				pick_list(sprite_names, Some(sprite.to_string()), Message::SetTagPreviewSprite),
				text_input("Animation String", animation, Message::SetTagPreviewAnimation)
			].spacing(5).align_items(Alignment::Center)
		);
	}

	let mut removescript = column![
		row![
			text("Remove Script"),
			checkbox("Auto", tag.removescript == RemoveScript::Auto, Message::SetTagRemoveScriptAuto)
		].spacing(20).align_items(Alignment::Center)
	].spacing(10);

	if tag.removescript != RemoveScript::Auto {
		let removescript_text = if let RemoveScript::Manual(removescript_string) = &tag.removescript {
			removescript_string.clone()
		} else {
			String::from("")
		};
		removescript = removescript.push(
			text_input("Remove Script", &removescript_text, Message::SetTagRemoveScript)
		);
	}

	column![
		column![
			row![
				text(format!("Agent Tag \"{}\"", &tag.name)).width(Length::Fill),
				button("x").on_press(Message::DeleteTag)
			].spacing(5).align_items(Alignment::Center),
			horizontal_rule(1),
		].padding([20, 20, 0, 20]).spacing(20),
		scrollable(
			column![
				row![
						text("Name").width(Length::FillPortion(1)),
						text_input("My Agent", &tag.name, Message::SetTagName).width(Length::FillPortion(3))
					].spacing(5).align_items(Alignment::Center),
				row![
						text("Description").width(Length::FillPortion(1)),
						text_input("Something that does some stuff", &tag.description, Message::SetTagDescription).width(Length::FillPortion(3))
					].spacing(5).align_items(Alignment::Center),
				row![
						text("Version").width(Length::FillPortion(1)),
						text_input("1.0", &tag.version, Message::SetTagVersion).width(Length::FillPortion(3))
					].spacing(5).align_items(Alignment::Center),
				row![
						text("Game").width(Length::FillPortion(1)),
						radio("C3 + DS", 0, supported_game_index, Message::SetTagSupportedGame).width(Length::FillPortion(1)),
						radio("C3 only", 1, supported_game_index, Message::SetTagSupportedGame).width(Length::FillPortion(1)),
						radio("DS only", 2, supported_game_index, Message::SetTagSupportedGame).width(Length::FillPortion(1))
					].spacing(5).align_items(Alignment::Center),
				preview,
				removescript
			].padding(20).spacing(20)
		).height(Length::Fill)
	]
}

pub fn listing(tag: &AgentTag) -> Column<Message> {
	let mut listing = column![
		row![
			button("+ Add File").on_press(Message::AddFile),
			button("+ Add Inline Catalogue").on_press(Message::AddInlineCatalogue)
		].spacing(5)
	].spacing(20);

	if !tag.scripts.is_empty() {
		listing = listing.push(script_view::list(&tag.scripts));
	}
	if !tag.sprites.is_empty() {
		listing = listing.push(sprite_view::list(&tag.sprites));
	}
	if !tag.backgrounds.is_empty() {
		listing = listing.push(background_view::list(&tag.backgrounds));
	}
	if !tag.sounds.is_empty() {
		listing = listing.push(sound_view::list(&tag.sounds));
	}
	if !tag.catalogues.is_empty() {
		listing = listing.push(catalogue_view::list(&tag.catalogues));
	}

	listing
}
