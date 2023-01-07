use crate::agent::tag::*;
use crate::ui::{ Main, SelectionType };
use crate::ui::dialogs::*;

#[derive(Debug, Clone)]
pub enum ScriptMessage {
	Select(usize),
	Remove,
	MoveUp(usize),
	MoveDown(usize),
	SetSupportedGame(usize),
}

pub fn check_script_message(main: &mut Main, message: ScriptMessage) {
	match message {
		ScriptMessage::Select(index) => {
			main.selection_type = SelectionType::Script(index);
		},

		ScriptMessage::Remove => {
			if confirm_remove_item("script") {
				if let Some(selected_tag) = main.selected_tag {
					if let SelectionType::Script(index) = main.selection_type {
						if let Tag::Agent(tag) = &mut main.tags[selected_tag] {
							tag.scripts.remove(index);
							main.selection_type = SelectionType::Tag;
							main.modified = true;
						}
					}
				}
			}
		},

		ScriptMessage::MoveUp(index) => {
			if let Some(selected_tag) = main.selected_tag {
				if let Tag::Agent(tag) = &mut main.tags[selected_tag] {
					tag.scripts.move_up(index);
					main.modified = true;
				}
			}
		},

		ScriptMessage::MoveDown(index) => {
			if let Some(selected_tag) = main.selected_tag {
				if let Tag::Agent(tag) = &mut main.tags[selected_tag] {
					tag.scripts.move_down(index);
					main.modified = true;
				}
			}
		},

		ScriptMessage::SetSupportedGame(new_supported_game) => {
			if let SelectionType::Script(index) = main.selection_type {
				if let Some(selected_tag) = main.selected_tag {
					if let Some(Tag::Agent(tag)) = &mut main.tags.get_mut(selected_tag) {
						if let Some(script) = tag.scripts.get_mut(index) {
							script.set_supported_game(new_supported_game);
							main.modified = true;
						}
					}
				}
			}
		},
	}
}
