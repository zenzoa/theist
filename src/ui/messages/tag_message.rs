use crate::agent::tag::*;
use crate::agent::agent_tag::*;
use crate::ui::{ Main, SelectionType };
use crate::ui::dialogs::*;
use crate::ui::messages::file_message::add_file;

#[derive(Debug, Clone)]
pub enum TagMessage {
	Add,
	Remove,
	Select(Option<usize>),
	SetName(String),
	SetDescription(String),
	SetVersion(String),
	SetSupportedGame(usize),
	SetPreviewAuto(bool),
	SetPreviewSprite(String),
	SetFemalePreviewSprite(String),
	SetMalePreviewSprite(String),
	SetPreviewAnimation(String),
	SetRemoveScriptAuto(bool),
	SetRemoveScript(String),
	AddFile,
	ConvertToEgg,
	ConvertToAgent
}

pub fn check_tag_message(main: &mut Main, message: TagMessage) {
	match message {
		TagMessage::Add => {
			main.tags.push(Tag::Agent(AgentTag::new(String::from("My Agent"))));
			main.selection_type = SelectionType::Tag;
			main.selected_tag = Some(main.tags.len() - 1);
			main.modified = true;
		},

		TagMessage::Remove => {
			if confirm_remove_tag() {
				if let Some(selected_tag) = &main.selected_tag {
					if selected_tag < &main.tags.len() {
						main.tags.remove(*selected_tag);
						main.selected_tag = if main.tags.is_empty() {
							None
						} else if selected_tag > &0 {
							Some(selected_tag - 1)
						} else {
							Some(0)
						};
						main.modified = true;
					}
				}
			}
		},

		TagMessage::Select(selected_tag) => {
			main.selected_tag = selected_tag;
			main.selection_type = SelectionType::Tag;
		},

		TagMessage::SetName(new_name) => {
			if let Some(selected_tag) = main.selected_tag {
				main.tags[selected_tag].set_name(new_name);
				main.modified = true;
			}
		},

		TagMessage::SetDescription(new_description) => {
			if let Some(selected_tag) = main.selected_tag {
				main.tags[selected_tag].set_description(new_description);
				main.modified = true;
			}
		},

		TagMessage::SetVersion(new_version) => {
			if let Some(selected_tag) = main.selected_tag {
				main.tags[selected_tag].set_version(new_version);
				main.modified = true;
			}
		},

		TagMessage::SetSupportedGame(new_supported_game) => {
			if let Some(selected_tag) = main.selected_tag {
				main.tags[selected_tag].set_supported_game(new_supported_game);
				main.modified = true;
			}
		},

		TagMessage::SetPreviewAuto(is_auto) => {
			if let Some(selected_tag) = main.selected_tag {
				main.tags[selected_tag].set_preview_auto(is_auto);
				main.modified = true;
			}
		},

		TagMessage::SetPreviewSprite(new_sprite) => {
			if let Some(selected_tag) = main.selected_tag {
				main.tags[selected_tag].set_preview_sprite(new_sprite);
				main.modified = true;
			}
		},

		TagMessage::SetFemalePreviewSprite(new_sprite) => {
			if let Some(selected_tag) = main.selected_tag {
				main.tags[selected_tag].set_female_preview_sprite(new_sprite);
				main.modified = true;
			}
		},

		TagMessage::SetMalePreviewSprite(new_sprite) => {
			if let Some(selected_tag) = main.selected_tag {
				main.tags[selected_tag].set_male_preview_sprite(new_sprite);
				main.modified = true;
			}
		},

		TagMessage::SetPreviewAnimation(new_animation) => {
			if let Some(selected_tag) = main.selected_tag {
				main.tags[selected_tag].set_preview_animation(new_animation);
				main.modified = true;
			}
		},

		TagMessage::SetRemoveScriptAuto(is_auto) => {
			if let Some(selected_tag) = main.selected_tag {
				main.tags[selected_tag].set_removescript_auto(is_auto);
				main.modified = true;
			}
		},

		TagMessage::SetRemoveScript(new_removescript) => {
			if let Some(selected_tag) = main.selected_tag {
				main.tags[selected_tag].set_removescript_string(new_removescript);
				main.modified = true;
			}
		},

		TagMessage::AddFile => {
			add_file(main);
		},

		TagMessage::ConvertToEgg => {
			if confirm_convert_tag("Egg tag") {
				if let Some(selected_tag) = main.selected_tag {
					if let Some(Tag::Agent(agent_tag)) = main.tags.get_mut(selected_tag) {
						let egg_tag = agent_tag.convert_to_egg();
						main.tags[selected_tag] = Tag::Egg(egg_tag);
						main.modified = true;
					}
				}
			}
		},

		TagMessage::ConvertToAgent => {
			if confirm_convert_tag("Agent tag") {
				if let Some(selected_tag) = main.selected_tag {
					if let Some(Tag::Egg(egg_tag)) = main.tags.get_mut(selected_tag) {
						let agent_tag = egg_tag.convert_to_agent();
						main.tags[selected_tag] = Tag::Agent(agent_tag);
						main.modified = true;
					}
				}
			}
		}
	}
}
