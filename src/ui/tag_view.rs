use crate::ui::*;
use crate::agent::*;
use crate::agent::tag::*;
use crate::agent::script::*;

use iced::widget::{ row, column, Column, text, text_input, button, radio, checkbox, scrollable, horizontal_rule };
use iced::{ Alignment, Length };

pub fn agent_listing(tag: &AgentTag) -> Column<Message> {
	let mut listing = column![
		row![
			button("+ Add File").on_press(Message::AddFile),
			button("+ Add Inline Catalogue").on_press(Message::AddInlineCatalogue)
		].spacing(5)
	].spacing(20);

	// Script list
	if !tag.scripts.is_empty() {
		let mut script_list = column![
			text("Scripts")
		].spacing(10);

		for (i, script) in tag.scripts.iter().enumerate() {
			let filename = match script {
				Script::File{ filename, .. } => filename
			};
			let buttons = if tag.scripts.len() > 1 {
				row![
					button("^").on_press(Message::MoveScriptUp(i)),
					button("v").on_press(Message::MoveScriptDown(i)),
					button("x").on_press(Message::DeleteScript(i))
				].spacing(5)
			} else {
				row![
					button("x").on_press(Message::DeleteScript(i))
				].spacing(5)
			};
			script_list = script_list.push(
				row![
					button(filename.title.as_str())
						.on_press(Message::SelectScript(i))
						.width(Length::Fill),
					buttons
				].spacing(5).align_items(Alignment::Center)
			);
		}

		listing = listing.push(script_list);
	}

	// Sprite list
	if !tag.sprites.is_empty() {
		let mut sprite_list = column![
			text("Sprites")
		].spacing(10);

		for (i, sprite) in tag.sprites.iter().enumerate() {
			let filename = match sprite {
				Sprite::C16{ filename } => filename,
				Sprite::Frames{ filename, .. } => filename
			};
			let buttons = if tag.sprites.len() > 1 {
				row![
					button("^").on_press(Message::MoveSpriteUp(i)),
					button("v").on_press(Message::MoveSpriteDown(i)),
					button("x").on_press(Message::DeleteSprite(i))
				].spacing(5)
			} else {
				row![
					button("x").on_press(Message::DeleteSprite(i))
				].spacing(5)
			};
			sprite_list = sprite_list.push(
				row![
					button(filename.title.as_str())
						.on_press(Message::SelectSprite(i))
						.width(Length::Fill),
					buttons
				].spacing(5).align_items(Alignment::Center)
			);
		}

		listing = listing.push(sprite_list);
	}

	// Background list
	if !tag.backgrounds.is_empty() {
		let mut background_list = column![
			text("Background Images")
		].spacing(10);

		for (i, background) in tag.backgrounds.iter().enumerate() {
			let filename = match background {
				Background::Png{ filename } => filename,
				Background::Blk{ filename } => filename
			};
			let buttons = if tag.backgrounds.len() > 1 {
				row![
					button("^").on_press(Message::MoveBackgroundUp(i)),
					button("v").on_press(Message::MoveBackgroundDown(i)),
					button("x").on_press(Message::DeleteBackground(i))
				].spacing(5)
			} else {
				row![
					button("x").on_press(Message::DeleteBackground(i))
				].spacing(5)
			};
			background_list = background_list.push(
				row![
					button(filename.title.as_str())
						.on_press(Message::SelectBackground(i))
						.width(Length::Fill),
					buttons
				].spacing(5).align_items(Alignment::Center)
			);
		}

		listing = listing.push(background_list);
	}

	// Sound list
	if !tag.sounds.is_empty() {
		let mut sound_list = column![
			text("Sounds")
		].spacing(10);

		for (i, sound) in tag.sounds.iter().enumerate() {
			let buttons = if tag.sounds.len() > 1 {
				row![
					button("^").on_press(Message::MoveSoundUp(i)),
					button("v").on_press(Message::MoveSoundDown(i)),
					button("x").on_press(Message::DeleteSound(i))
				].spacing(5)
			} else {
				row![
					button("x").on_press(Message::DeleteSound(i))
				].spacing(5)
			};
			sound_list = sound_list.push(
				row![
					button(sound.filename.title.as_str())
						.on_press(Message::SelectSound(i))
						.width(Length::Fill),
					buttons
				].spacing(5).align_items(Alignment::Center)
			);
		}

		listing = listing.push(sound_list);
	}

	// Catalogue list
	if !tag.catalogues.is_empty() {
		let mut catalogue_list = column![
			text("Catalogues")
		].spacing(10);

		for (i, catalogue) in tag.catalogues.iter().enumerate() {
			let filename = match catalogue {
				Catalogue::File{ filename } => filename,
				Catalogue::Inline{ filename, .. } => filename
			};
			let buttons = if tag.catalogues.len() > 1 {
				row![
					button("^").on_press(Message::MoveCatalogueUp(i)),
					button("v").on_press(Message::MoveCatalogueDown(i)),
					button("x").on_press(Message::DeleteCatalogue(i))
				].spacing(5)
			} else {
				row![
					button("x").on_press(Message::DeleteCatalogue(i))
				].spacing(5)
			};
			catalogue_list = catalogue_list.push(
				row![
					button(filename.title.as_str())
						.on_press(Message::SelectCatalogue(i))
						.width(Length::Fill),
					buttons
				].spacing(5).align_items(Alignment::Center)
			);
		}

		listing = listing.push(catalogue_list);
	}

	listing
}

pub fn agent_properties(tag: &AgentTag) -> Column<Message> {
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

	if let Preview::Manual { sprite, animation } = &tag.preview {
		preview = preview.push(
			row![
				text_input("Sprite Name", sprite, Message::SetTagPreviewSprite),
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
				text(format!("Tag \"{}\"", &tag.name)).width(Length::Fill),
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
