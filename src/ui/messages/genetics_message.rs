use crate::agent::tag::*;
use crate::ui::{ Main, SelectionType };
use crate::ui::dialogs::*;

#[derive(Debug, Clone)]
pub enum GeneticsMessage {
	Select(usize),
	Remove,
	MoveUp(usize),
	MoveDown(usize),
}

pub fn check_genetics_message(main: &mut Main, message: GeneticsMessage) {
	match message {
		GeneticsMessage::Select(index) => {
			main.selection_type = SelectionType::Genetics(index);
		},

		GeneticsMessage::Remove => {
			if confirm_remove_item("genetics file") {
				if let Some(selected_tag) = main.selected_tag {
					if let SelectionType::Genetics(index) = main.selection_type {
						if let Tag::Egg(tag) = &mut main.tags[selected_tag] {
							tag.genetics.remove(index);
							main.selection_type = SelectionType::Tag;
							main.modified = true;
						}
					}
				}
			}
		},

		GeneticsMessage::MoveUp(index) => {
			if let Some(selected_tag) = main.selected_tag {
				if let Tag::Egg(tag) = &mut main.tags[selected_tag] {
					tag.genetics.move_up(index);
					main.modified = true;
				}
			}
		},

		GeneticsMessage::MoveDown(index) => {
			if let Some(selected_tag) = main.selected_tag {
				if let Tag::Egg(tag) = &mut main.tags[selected_tag] {
					tag.genetics.move_down(index);
					main.modified = true;
				}
			}
		},
	}
}
