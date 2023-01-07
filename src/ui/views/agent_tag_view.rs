use crate::ui::{ SelectionType };
use crate::ui::icons::*;
use crate::ui::messages::Message;
use crate::ui::messages::tag_message::TagMessage;
use crate::ui::messages::catalogue_message::CatalogueMessage;
use crate::ui::views::{ script_view, sprite_view, background_view, sound_view, catalogue_view };
use crate::agent::*;
use crate::agent::agent_tag::*;
use crate::agent::sprite::*;

use iced::widget::{ row, column, Column, text, text_input, button, radio, checkbox, pick_list, scrollable, horizontal_rule };
use iced::{ alignment, Alignment, Length, theme };

pub fn properties(tag: &AgentTag) -> Column<Message> {
	let supported_game_index = match tag.supported_game {
		SupportedGame::C3DS => Some(0),
		SupportedGame::C3 => Some(1),
		SupportedGame::DS => Some(2)
	};

	let mut preview = column![
		row![
			text("Injector Preview"),
			checkbox("Auto", tag.preview == Preview::Auto, |x| Message::Tag(TagMessage::SetPreviewAuto(x)))
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
				pick_list(sprite_names, Some(sprite.to_string()), |x| Message::Tag(TagMessage::SetPreviewSprite(x))),
				text_input("Animation String", animation, |x| Message::Tag(TagMessage::SetPreviewAnimation(x)))
			].spacing(5).align_items(Alignment::Center)
		);
	}

	let mut removescript = column![
		row![
			text("Remove Script"),
			checkbox("Auto", tag.removescript == RemoveScript::Auto, |x| Message::Tag(TagMessage::SetRemoveScriptAuto(x)))
		].spacing(20).align_items(Alignment::Center)
	].spacing(10);

	if tag.removescript != RemoveScript::Auto {
		let removescript_text = if let RemoveScript::Manual(removescript_string) = &tag.removescript {
			removescript_string.clone()
		} else {
			String::from("")
		};
		removescript = removescript.push(
			text_input("Remove Script", &removescript_text, |x| Message::Tag(TagMessage::SetRemoveScript(x)))
		);
	}

	let mut convert_to_egg = column![].spacing(20);

	if tag.scripts.is_empty() && tag.catalogues.is_empty() && tag.sounds.is_empty() && tag.backgrounds.is_empty() {
		convert_to_egg = convert_to_egg.push(horizontal_rule(1));
		convert_to_egg = convert_to_egg.push(
			button(text("Convert to Egg Tag").width(Length::Fill).horizontal_alignment(alignment::Horizontal::Center))
				.on_press(Message::Tag(TagMessage::ConvertToEgg))
				.style(theme::Button::Secondary)
				.width(Length::FillPortion(1))
		);
	}

	column![
		column![
			row![
				tag_icon(),
				text(format!("Agent Tag \"{}\"", &tag.name)).width(Length::Fill),
				button(delete_icon())
					.on_press(Message::Tag(TagMessage::Remove))
					.style(theme::Button::Destructive)
			].spacing(5).align_items(Alignment::Center),
			horizontal_rule(1),
		].padding([30, 30, 0, 30]).spacing(20),
		scrollable(
			column![
				row![
						text("Name").width(Length::FillPortion(1)),
						text_input("My Agent", &tag.name, |x| Message::Tag(TagMessage::SetName(x)))
							.width(Length::FillPortion(3))
					].spacing(5).align_items(Alignment::Center),
				row![
						text("Description").width(Length::FillPortion(1)),
						text_input("Something descriptive", &tag.description, |x| Message::Tag(TagMessage::SetDescription(x)))
							.width(Length::FillPortion(3))
					].spacing(5).align_items(Alignment::Center),
				row![
						text("Version").width(Length::FillPortion(1)),
						text_input("1.0", &tag.version, |x| Message::Tag(TagMessage::SetVersion(x))).width(Length::FillPortion(3))
					].spacing(5).align_items(Alignment::Center),
				row![
						text("Game").width(Length::FillPortion(1)),
						radio("C3 + DS", 0, supported_game_index, |x| Message::Tag(TagMessage::SetSupportedGame(x)))
							.width(Length::FillPortion(1)),
						radio("C3 only", 1, supported_game_index, |x| Message::Tag(TagMessage::SetSupportedGame(x)))
							.width(Length::FillPortion(1)),
						radio("DS only", 2, supported_game_index, |x| Message::Tag(TagMessage::SetSupportedGame(x)))
							.width(Length::FillPortion(1))
					].spacing(5).align_items(Alignment::Center),
				preview,
				removescript,
				convert_to_egg
			].padding(30).spacing(20)
		).height(Length::Fill)
	]
}

pub fn listing(tag: &AgentTag, selection_type: SelectionType) -> Column<Message> {
	let mut listing = column![
		row![
			button(row![add_icon(), text("Add File")].spacing(5))
				.on_press(Message::Tag(TagMessage::AddFile))
				.style(theme::Button::Secondary),
			button(row![add_icon(), text("Add Inline Catalogue")].spacing(5))
				.on_press(Message::Catalogue(CatalogueMessage::AddInlineCatalogue))
				.style(theme::Button::Secondary)
		].spacing(5)
	].spacing(20);

	if !tag.scripts.is_empty() {
		let script_index = if let SelectionType::Script(index) = selection_type { Some(index) } else { None };
		listing = listing.push(script_view::list(&tag.scripts, script_index));
	}
	if !tag.sprites.is_empty() {
		let sprite_index = if let SelectionType::Sprite(index) = selection_type { Some(index) } else { None };
		listing = listing.push(sprite_view::list(&tag.sprites, sprite_index));
	}
	if !tag.backgrounds.is_empty() {
		let background_index = if let SelectionType::Background(index) = selection_type { Some(index) } else { None };
		listing = listing.push(background_view::list(&tag.backgrounds, background_index));
	}
	if !tag.sounds.is_empty() {
		let sound_index = if let SelectionType::Sound(index) = selection_type { Some(index) } else { None };
		listing = listing.push(sound_view::list(&tag.sounds, sound_index));
	}
	if !tag.catalogues.is_empty() {
		let catalogue_index = if let SelectionType::Catalogue(index) = selection_type { Some(index) } else { None };
		listing = listing.push(catalogue_view::list(&tag.catalogues, catalogue_index));
	}

	listing
}
