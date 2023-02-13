use crate::ui::Main;
use crate::agent::file::CreaturesFile;

#[derive(Debug, Clone)]
pub enum BodyDataMessage {
	SetName(String)
}

pub fn check_bodydata_message(main: &mut Main, message: BodyDataMessage) {
	match message {
		BodyDataMessage::SetName(new_name) => {
			if let Some(CreaturesFile::BodyData(bodydata)) = main.get_selected_file_mut() {
				bodydata.output_filename = format!("{}.att", new_name);
				main.modified = true;
			}
		}
	}
}

