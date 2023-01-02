use crate::agent::tag::*;
use crate::ui::{ Main, SelectionType };
use crate::ui::dialogs::*;

use std::path::PathBuf;
use rfd::FileDialog;

#[derive(Debug, Clone)]
pub enum SpriteMessage {
	Select(usize),
	Remove(usize),
	MoveUp(usize),
	MoveDown(usize),
	SetName(String),
	ConvertToBackground,
	AddFrame,
	RemoveFrame(usize),
	MoveFrameUp(usize),
	MoveFrameDown(usize),
}

pub fn check_sprite_message(main: &mut Main, message: SpriteMessage) {
	match message {
		SpriteMessage::Select(index) => {
			main.selection_type = SelectionType::Sprite(index);
		},

		SpriteMessage::Remove(index) => {
			if confirm_remove_item("sprite") {
				if let Some(selected_tag) = main.selected_tag {
					match &mut main.tags[selected_tag] {
						Tag::Agent(tag) => tag.sprites.remove(index),
						Tag::Egg(tag) => tag.sprites.remove(index),
						_ => ()
					}
					main.selection_type = SelectionType::Tag;
					main.modified = true;
				}
			}
		},

		SpriteMessage::MoveUp(index) => {
			if let Some(selected_tag) = main.selected_tag {
				match &mut main.tags[selected_tag] {
					Tag::Agent(tag) => tag.sprites.move_up(index),
					Tag::Egg(tag) => tag.sprites.move_up(index),
					_ => ()
				}
				main.modified = true;
			}
		},

		SpriteMessage::MoveDown(index) => {
			if let Some(selected_tag) = main.selected_tag {
				match &mut main.tags[selected_tag] {
					Tag::Agent(tag) => tag.sprites.move_down(index),
					Tag::Egg(tag) => tag.sprites.move_down(index),
					_ => ()
				}
				main.modified = true;
			}
		},

		SpriteMessage::SetName(new_name) => {
			if let SelectionType::Sprite(index) = main.selection_type {
				if let Some(selected_tag) = main.selected_tag {
					let mut sprite = match &mut main.tags[selected_tag] {
						Tag::Agent(tag) => tag.sprites.get_mut(index),
						Tag::Egg(tag) => tag.sprites.get_mut(index),
						_ => None
					};
					if let Some(sprite) = &mut sprite {
						sprite.set_name(new_name);
						main.modified = true;
					}
				}
			}
		},

		SpriteMessage::ConvertToBackground => {
			if let Some(selected_tag) = main.selected_tag {
				if let SelectionType::Sprite(index) = main.selection_type {
					if let Tag::Agent(tag) = &mut main.tags[selected_tag] {
						if let Some(sprite) = tag.sprites.get(index) {
							if let Some(new_background) = sprite.convert_to_background() {
								tag.backgrounds.push(new_background);
								main.selection_type = SelectionType::Background(tag.backgrounds.len() - 1);
								tag.sprites.remove(index);
								main.modified = true;
							}
						}
					}
				}
			}
		},

		SpriteMessage::AddFrame => {
			add_sprite_frame(main);
		},

		SpriteMessage::RemoveFrame(frame_index) => {
			if confirm_remove_item("sprite frame") {
				if let Some(selected_tag) = main.selected_tag {
					if let SelectionType::Sprite(sprite_index) = main.selection_type {
						let mut sprite = match &mut main.tags[selected_tag] {
							Tag::Agent(tag) => tag.sprites.get_mut(sprite_index),
							Tag::Egg(tag) => tag.sprites.get_mut(sprite_index),
							_ => None
						};
						if let Some(sprite) = &mut sprite {
							sprite.remove_frame(frame_index);
							main.modified = true;
						}
					}
				}
			}
		},

		SpriteMessage::MoveFrameUp(frame_index) => {
			if let Some(selected_tag) = main.selected_tag {
				if let SelectionType::Sprite(sprite_index) = main.selection_type {
					let mut sprite = match &mut main.tags[selected_tag] {
						Tag::Agent(tag) => tag.sprites.get_mut(sprite_index),
						Tag::Egg(tag) => tag.sprites.get_mut(sprite_index),
						_ => None
					};
					if let Some(sprite) = &mut sprite {
						sprite.move_frame_up(frame_index);
						main.modified = true;
					}
				}
			}
		},

		SpriteMessage::MoveFrameDown(frame_index) => {
			if let Some(selected_tag) = main.selected_tag {
				if let SelectionType::Sprite(sprite_index) = main.selection_type {
					let mut sprite = match &mut main.tags[selected_tag] {
						Tag::Agent(tag) => tag.sprites.get_mut(sprite_index),
						Tag::Egg(tag) => tag.sprites.get_mut(sprite_index),
						_ => None
					};
					if let Some(sprite) = &mut sprite {
						sprite.move_frame_down(frame_index);
						main.modified = true;
					}
				}
			}
		},
	}
}

pub fn add_sprite_frame(main: &mut Main) {
	let file = FileDialog::new()
		.add_filter("Creatures Files", &["cos", "c16", "blk", "wav", "catalogue", "png"])
		.set_directory(&main.path)
		.pick_file();
	if let Some(file_path) = file {
		add_sprite_frame_from_path(main, file_path, false);
	}
}

pub fn add_sprite_frame_from_path(main: &mut Main, file_path: PathBuf, file_dropped: bool) -> bool {
	if let Some(path) = file_path.parent() {
		if let Some(filename) = file_path.file_name() {
			if let Some(extension) = file_path.extension() {
				let path = path.to_string_lossy().into_owned() + "/";
				let filename = filename.to_string_lossy().into_owned();
				let extension = extension.to_string_lossy().into_owned();
				if main.path == path {
					if extension == "png" {
						if let Some(selected_tag) = main.selected_tag {
							if let SelectionType::Sprite(index) = main.selection_type {
								let mut sprite = match &mut main.tags[selected_tag] {
									Tag::Agent(tag) => tag.sprites.get_mut(index),
									Tag::Egg(tag) => tag.sprites.get_mut(index),
									_ => None
								};
								if let Some(sprite) = &mut sprite {
									sprite.add_frame(filename.as_str());
									return true;
								}
							}
						}
					} else if !file_dropped {
						alert_wrong_filetype("PNG");
					}
				} else if !file_dropped {
					alert_wrong_folder();
				}
			}
		}
	}
	false
}
