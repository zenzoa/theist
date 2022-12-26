use crate::ui::*;
use crate::agent::*;
use crate::agent::tag::*;
use crate::agent::script::*;

use iced::widget::{ row, column, Column, text, text_input, button, radio, checkbox, vertical_space };
use iced::{ Alignment, Length };

pub fn agent_listing(tag: &AgentTag) -> Column<Message> {
	let mut listing = column![].spacing(40);

	// Script list
	let mut script_list = column![
			row![
					text("Scripts"),
					button("+").on_press(Message::AddScript)
				]
				.spacing(5)
				.align_items(Alignment::Center)
		]
		.spacing(10);

	for script in &tag.scripts {
		match script {
			Script::File { filename, supported_game } => {
				script_list = script_list.push(
					button(filename.title.as_str())
				);
			}
		}
	}

	listing = listing.push(script_list);

	// Sprite + background list
	let mut sprite_list = column![
			row![
					text("Sprites"),
					button("+").on_press(Message::AddSprite)
				]
				.spacing(5)
				.align_items(Alignment::Center)
		]
		.spacing(10);

	for sprite in &tag.sprites {
		let sprite_name = match sprite {
			Sprite::C16{ filename } => filename,
			Sprite::Frames{ filename, .. } => filename
		};
		sprite_list = sprite_list.push(
			button(sprite_name.title.as_str())
		);
	}

	for background in &tag.backgrounds {
		let background_name = match background {
			Background::Png{ filename } => filename,
			Background::Blk{ filename } => filename
		};
		sprite_list = sprite_list.push(
			button(background_name.title.as_str())
		);
	}

	listing = listing.push(sprite_list);

	// Sound list
	let mut sound_list = column![
			row![
					text("Sounds"),
					button("+").on_press(Message::AddSound)
				]
				.spacing(5)
				.align_items(Alignment::Center)
		]
		.spacing(10);

	for sound in &tag.sounds {
		sound_list = sound_list.push(
			button(sound.filename.title.as_str())
		);
	}

	listing = listing.push(sound_list);

	// Catalogue list
	let mut catalogue_list = column![
			row![
					text("Catalogues"),
					button("+").on_press(Message::AddCatalogue)
				]
				.spacing(5)
				.align_items(Alignment::Center)
		]
		.spacing(10);

	for catalogue in &tag.catalogues {
		let catalogue_name = match catalogue {
			Catalogue::File{ filename } => filename,
			Catalogue::Inline{ filename, .. } => filename
		};
		catalogue_list = catalogue_list.push(
			button(catalogue_name.title.as_str())
		);
	}

	listing = listing.push(catalogue_list);

	listing
}

pub fn agent_properties(tag: &AgentTag) -> Column<Message> {
	let supported_game = match tag.supported_game {
		SupportedGame::C3DS => Some(0),
		SupportedGame::C3 => Some(1),
		SupportedGame::DS => Some(2)
	};

	let mut preview = column![
		row![
				text("Injector Preview"),
				checkbox("Auto", tag.preview == Preview::Auto, Message::SetTagPreviewAuto)
			]
			.spacing(20)
			.align_items(Alignment::Center)
		]
		.spacing(10);

	if let Preview::Manual { sprite, animation } = &tag.preview {
		preview = preview.push(
			row![
					text_input("Sprite Name", &sprite, Message::SetTagPreviewSprite),
					text_input("Animation String", &animation, Message::SetTagPreviewAnimation)
				]
				.spacing(5)
				.align_items(Alignment::Center)
		);
	}

	let mut removescript = column![
		row![
				text("Remove Script"),
				checkbox("Auto", tag.removescript == RemoveScript::Auto, Message::SetTagRemoveScriptAuto)
			]
			.spacing(20)
			.align_items(Alignment::Center)
		]
		.spacing(10);

	if tag.removescript != RemoveScript::Auto {
		let removescript_text = if let RemoveScript::Manual(removescript_string) = &tag.removescript {
			removescript_string.clone().to_string()
		} else {
			String::from("")
		};
		removescript = removescript.push(
			text_input("Remove Script", &removescript_text, Message::SetTagRemoveScript)
		);
	}

	column![
		text(format!("Properties for {}", &tag.name)),
		row![
				text("Name").width(Length::FillPortion(1)),
				text_input("My Agent", &tag.name, Message::SetTagName).width(Length::FillPortion(3))
			]
			.spacing(5)
			.align_items(Alignment::Center),
		row![
				text("Description").width(Length::FillPortion(1)),
				text_input("Something that does some stuff", &tag.description, Message::SetTagDescription).width(Length::FillPortion(3))
			]
			.spacing(5)
			.align_items(Alignment::Center),
		row![
				text("Version").width(Length::FillPortion(1)),
				text_input("1.0", &tag.version, Message::SetTagVersion).width(Length::FillPortion(3))
			]
			.spacing(5)
			.align_items(Alignment::Center),
		row![
				text("Game").width(Length::FillPortion(1)),
				radio("C3 + DS", 0, supported_game, Message::SetTagSupportedGame).width(Length::FillPortion(1)),
				radio("C3 only", 1, supported_game, Message::SetTagSupportedGame).width(Length::FillPortion(1)),
				radio("DS only", 2, supported_game, Message::SetTagSupportedGame).width(Length::FillPortion(1))
			]
			.spacing(5)
			.align_items(Alignment::Center),
		preview,
		removescript,
		vertical_space(Length::Fill),
		button("Delete").on_press(Message::DeleteTag)
	]
	.spacing(20)
}
