use rfd::{ MessageDialog, MessageLevel, MessageButtons };

pub fn confirm_discard_changes() -> bool {
	MessageDialog::new()
		.set_title("File modified")
		.set_description("Do you want to continue anyway and lose any unsaved work?")
		.set_level(MessageLevel::Warning)
		.set_buttons(MessageButtons::YesNo)
		.show()
}

pub fn confirm_remove_tag() -> bool {
	MessageDialog::new()
		.set_title("Remove tag?")
		.set_description("Are you sure you want to remove this tag? It won't delete any files it refers to, but you will lose all info stored in the tag itself.")
		.set_level(MessageLevel::Warning)
		.set_buttons(MessageButtons::YesNo)
		.show()
}

pub fn confirm_remove_item(name: &str) -> bool {
	MessageDialog::new()
		.set_title(format!("Remove {}?", name).as_str())
		.set_description("Are you sure you want to remove it from this tag? It won't delete any files on your hard drive.")
		.set_level(MessageLevel::Warning)
		.set_buttons(MessageButtons::YesNo)
		.show()
}

pub fn confirm_remove_frame() -> bool {
	MessageDialog::new()
		.set_title("Remove sprite frame?")
		.set_description("Are you sure you want to remove it? It won't delete any files on your hard drive.")
		.set_level(MessageLevel::Warning)
		.set_buttons(MessageButtons::YesNo)
		.show()
}

pub fn confirm_remove_entry() -> bool {
	MessageDialog::new()
		.set_title("Remove catalogue entry?")
		.set_description("Are you sure you want to remove it? It won't delete any files on your hard drive.")
		.set_level(MessageLevel::Warning)
		.set_buttons(MessageButtons::YesNo)
		.show()
}

pub fn confirm_extract(file_count: usize) -> bool {
	MessageDialog::new()
		.set_title("Extract files?")
		.set_description(format!("Do you want to extract the {} files associated with this agent?", file_count).as_str())
		.set_level(MessageLevel::Warning)
		.set_buttons(MessageButtons::YesNo)
		.show()
}

pub fn confirm_exit() -> bool {
	MessageDialog::new()
		.set_title("File modified")
		.set_description("Do you want to exit and lose any unsaved work?")
		.set_level(MessageLevel::Warning)
		.set_buttons(MessageButtons::YesNo)
		.show()
}

pub fn alert_wrong_folder() -> bool {
	MessageDialog::new()
		.set_title("Wrong folder")
		.set_description("Unable to load file. All files must be located relative to the folder the theist file is in.")
		.set_level(MessageLevel::Warning)
		.set_buttons(MessageButtons::Ok)
		.show()
}

pub fn alert_missing_data() -> bool {
	MessageDialog::new()
		.set_title("Missing data")
		.set_description("Unable to compile agent. One or more files is missing.")
		.set_level(MessageLevel::Error)
		.set_buttons(MessageButtons::Ok)
		.show()
}
