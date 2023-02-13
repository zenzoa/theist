use crate::ui::Main;
use crate::agent::file::CreaturesFile;

#[derive(Debug, Clone)]
pub enum ScriptMessage {
	SetName(String)
}

pub fn check_script_message(main: &mut Main, message: ScriptMessage) {
	match message {
		ScriptMessage::SetName(new_name) => {
			if let Some(CreaturesFile::Script(script)) = main.get_selected_file_mut() {
				script.output_filename = format!("{}.cos", new_name);
				main.modified = true;
			}
		}
	}
}

