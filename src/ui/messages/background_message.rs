use crate::agent::tag::*;
use crate::ui::{ Main, SelectionType };
use crate::ui::dialogs::*;

#[derive(Debug, Clone)]
pub enum BackgroundMessage {
	Select(usize),
	Remove(usize),
	MoveUp(usize),
	MoveDown(usize),
	ConvertToSprite,
}

pub fn check_background_message(main: &mut Main, message: BackgroundMessage) {
	match message {
		BackgroundMessage::Select(index) => {
			main.selection_type = SelectionType::Background(index);
		},

		BackgroundMessage::Remove(index) => {
			if confirm_remove_item("background") {
				if let Some(selected_tag) = main.selected_tag {
					if let Tag::Agent(tag) = &mut main.tags[selected_tag] {
						tag.backgrounds.remove(index);
						main.selection_type = SelectionType::Tag;
						main.modified = true;
					}
				}
			}
		},

		BackgroundMessage::MoveUp(index) => {
			if let Some(selected_tag) = main.selected_tag {
				if let Tag::Agent(tag) = &mut main.tags[selected_tag] {
					tag.backgrounds.move_up(index);
					main.modified = true;
				}
			}
		},

		BackgroundMessage::MoveDown(index) => {
			if let Some(selected_tag) = main.selected_tag {
				if let Tag::Agent(tag) = &mut main.tags[selected_tag] {
					tag.backgrounds.move_down(index);
					main.modified = true;
				}
			}
		},

		BackgroundMessage::ConvertToSprite => {
			if let Some(selected_tag) = main.selected_tag {
				if let SelectionType::Background(index) = main.selection_type {
					if let Tag::Agent(tag) = &mut main.tags[selected_tag] {
						if let Some(background) = tag.backgrounds.get(index) {
							if let Some(new_sprite) = background.convert_to_sprite() {
								tag.sprites.push(new_sprite);
								main.selection_type = SelectionType::Sprite(tag.sprites.len() - 1);
								tag.backgrounds.remove(index);
								main.modified = true;
							}
						}
					}
				}
			}
		},
	}
}
