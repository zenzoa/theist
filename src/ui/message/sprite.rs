use crate::ui::Main;
use crate::ui::dialog::*;
use crate::agent::file::CreaturesFile;
use crate::agent::sprite::{ Sprite, SpriteFrame };
use crate::file_helper;

use std::fs;
use rfd::FileDialog;
use std::error::Error;
use bytes::Bytes;

#[derive(Debug, Clone)]
pub enum SpriteMessage {
	SetName(String),
	SetExtension(String),
	AddFrame,
	RemoveFrame(usize),
	MoveFrameUp(usize),
	MoveFrameDown(usize)
}

pub fn check_sprite_message(main: &mut Main, message: SpriteMessage) {
	let main_path = main.path.clone();
	if let Some(CreaturesFile::Sprite(sprite)) = main.get_selected_file_mut() {
		match message {
			SpriteMessage::SetName(new_name) => {
				match sprite {
					Sprite::Raw{ output_filename, .. } => {
						let extension = file_helper::extension(output_filename);
						*output_filename = format!("{}.{}", new_name, extension);
					},
					Sprite::Png{ output_filename, .. } => {
						let extension = file_helper::extension(output_filename);
						*output_filename = format!("{}.{}", new_name, extension);
					}
				}
				main.modified = true;
			},

			SpriteMessage::SetExtension(new_extension) => {
				if let Sprite::Png{ output_filename, .. } = sprite {
					let title = file_helper::title(output_filename);
					*output_filename = format!("{}.{}", title, new_extension);
					main.modified = true;
				}
			},

			SpriteMessage::AddFrame => {
				let file = FileDialog::new()
					.add_filter("PNG Files", &["png"])
					.set_directory(&main_path)
					.pick_file();
				if let Some(filepath) = file {
					match make_sprite_frame(&main_path, filepath.to_string_lossy().into_owned()) {
						Ok(new_frame) => if let Some(new_frame) = new_frame {
							sprite.add_frame(new_frame);
							main.modified = true;
						},
						Err(why) => main.add_alert(&format!("ERROR: Unable to add frame: {}", why), true)
					}
				}
			},

			SpriteMessage::RemoveFrame(frame_index) => {
				if confirm_remove_frame() && sprite.remove_frame(frame_index) {
					main.modified = true;
				}
			},

			SpriteMessage::MoveFrameUp(frame_index) => {
				if sprite.move_frame_up(frame_index) {
					main.modified = true;
				}
			},

			SpriteMessage::MoveFrameDown(frame_index) => {
				if sprite.move_frame_down(frame_index) {
					main.modified = true;
				}
			}
		}
	}
}

fn make_sprite_frame(main_path: &String, filepath: String) -> Result<Option<SpriteFrame>, Box<dyn Error>> {
	let path = file_helper::path(&filepath);
	let filename = file_helper::filename(&filepath);

	match path.strip_prefix(main_path) {
		Some(relative_path) => {
			let input_filename = format!("{}{}", relative_path, filename);
			let contents = fs::read(&filepath)?;
			let frame_data = &mut Bytes::from(contents);
			let new_frame = SpriteFrame::new_from_data(&input_filename, frame_data)?;
			Ok(Some(new_frame))
		},
		None => {
			alert_wrong_folder();
			Ok(None)
		}
	}
}
