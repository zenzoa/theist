use crate::agent::tag::*;
use crate::ui::{ Main, SelectionType };
use crate::ui::dialogs::*;

#[derive(Debug, Clone)]
pub enum SoundMessage {
	Select(usize),
	Remove,
	MoveUp(usize),
	MoveDown(usize),
}

pub fn check_sound_message(main: &mut Main, message: SoundMessage) {
	match message {
		SoundMessage::Select(index) => {
			main.selection_type = SelectionType::Sound(index);
		},

		SoundMessage::Remove => {
			if confirm_remove_item("sound") {
				if let Some(selected_tag) = main.selected_tag {
					if let SelectionType::Sound(index) = main.selection_type {
						if let Tag::Agent(tag) = &mut main.tags[selected_tag] {
							tag.sounds.remove(index);
							main.selection_type = SelectionType::Tag;
							main.modified = true;
						}
					}
				}
			}
		},

		SoundMessage::MoveUp(index) => {
			if let Some(selected_tag) = main.selected_tag {
				if let Tag::Agent(tag) = &mut main.tags[selected_tag] {
					tag.sounds.move_up(index);
					main.modified = true;
				}
			}
		},

		SoundMessage::MoveDown(index) => {
			if let Some(selected_tag) = main.selected_tag {
				if let Tag::Agent(tag) = &mut main.tags[selected_tag] {
					tag.sounds.move_down(index);
					main.modified = true;
				}
			}
		},
	}
}
