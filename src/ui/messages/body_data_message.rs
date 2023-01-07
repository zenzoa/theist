use crate::agent::tag::*;
use crate::ui::{ Main, SelectionType };
use crate::ui::dialogs::*;

#[derive(Debug, Clone)]
pub enum BodyDataMessage {
	Select(usize),
	Remove,
	MoveUp(usize),
	MoveDown(usize),
}

pub fn check_body_data_message(main: &mut Main, message: BodyDataMessage) {
	match message {
		BodyDataMessage::Select(index) => {
			main.selection_type = SelectionType::BodyData(index);
		},

		BodyDataMessage::Remove => {
			if confirm_remove_item("body data file") {
				if let Some(selected_tag) = main.selected_tag {
					if let SelectionType::BodyData(index) = main.selection_type {
						if let Tag::Egg(tag) = &mut main.tags[selected_tag] {
							tag.body_data.remove(index);
							main.selection_type = SelectionType::Tag;
							main.modified = true;
						}
					}
				}
			}
		},

		BodyDataMessage::MoveUp(index) => {
			if let Some(selected_tag) = main.selected_tag {
				if let Tag::Egg(tag) = &mut main.tags[selected_tag] {
					tag.body_data.move_up(index);
					main.modified = true;
				}
			}
		},

		BodyDataMessage::MoveDown(index) => {
			if let Some(selected_tag) = main.selected_tag {
				if let Tag::Egg(tag) = &mut main.tags[selected_tag] {
					tag.body_data.move_down(index);
					main.modified = true;
				}
			}
		},
	}
}
