use crate::ui::Main;
use crate::agent::file::CreaturesFile;
use crate::file_helper;

#[derive(Debug, Clone)]
pub enum GeneticsMessage {
	SetName(String)
}

pub fn check_genetics_message(main: &mut Main, message: GeneticsMessage) {
	match message {
		GeneticsMessage::SetName(new_name) => {
			if let Some(CreaturesFile::Genetics(genetics)) = main.get_selected_file_mut() {
				let extension = file_helper::extension(&genetics.output_filename);
				genetics.output_filename = format!("{}.{}", new_name, extension);
				main.modified = true;
			}
		}
	}
}

