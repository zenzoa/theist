use crate::ui::Main;
use crate::agent::file::CreaturesFile;

#[derive(Debug, Clone)]
pub enum SoundMessage {
	SetName(String)
}

pub fn check_sound_message(main: &mut Main, message: SoundMessage) {
	if let Some(CreaturesFile::Sound(sound)) = main.get_selected_file_mut() {
		match message {
			SoundMessage::SetName(new_name) => {
				sound.output_filename = format!("{}.wav", new_name);
				main.modified = true;
			}
		}
	}
}

