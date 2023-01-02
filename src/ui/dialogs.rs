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

pub fn confirm_convert_tag(name: &str) -> bool {
	MessageDialog::new()
		.set_title("Convert tag?")
		.set_description(format!("Are you sure you want to convert this tag to an {}?", name).as_str())
		.set_level(MessageLevel::Warning)
		.set_buttons(MessageButtons::YesNo)
		.show()
}

pub fn confirm_remove_item(name: &str) -> bool {
	MessageDialog::new()
		.set_title(format!("Remove {}?", name).as_str())
		.set_description(format!("Are you sure you want to remove this {} from the tag? It won't delete any files it refers to.", name).as_str())
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
		.set_description("Unable to load file. All files must be located in the same folder.")
		.set_level(MessageLevel::Warning)
		.set_buttons(MessageButtons::Ok)
		.show()
}

pub fn alert_wrong_filetype(extension: &str) -> bool {
	MessageDialog::new()
		.set_title("Wrong file type")
		.set_description(format!("Unable to load file. File must be of type {}.", extension).as_str())
		.set_level(MessageLevel::Warning)
		.set_buttons(MessageButtons::Ok)
		.show()
}

pub fn alert_wrong_genetics_title() -> bool {
	MessageDialog::new()
		.set_title("GEN and GNO have different names")
		.set_description("Unable to load file. An egg tag's GEN and GNO files must have the same name.")
		.set_level(MessageLevel::Warning)
		.set_buttons(MessageButtons::Ok)
		.show()
}

pub fn alert_too_many_genetics_files() -> bool {
	MessageDialog::new()
		.set_title("Too many genetics files")
		.set_description("Unable to load file. An egg tag can only have two genetics files, one GEN and one GNO.")
		.set_level(MessageLevel::Warning)
		.set_buttons(MessageButtons::Ok)
		.show()
}
