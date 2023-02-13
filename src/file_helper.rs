use std::fs::File;
use std::path::{ PathBuf, MAIN_SEPARATOR };

pub fn path(filename: &str) -> String {
	match PathBuf::from(filename).parent() {
		Some(parent) => format!("{}{}", parent.to_string_lossy().into_owned(), MAIN_SEPARATOR),
		None => "".to_string()
	}
}

pub fn title(filename: &str) -> String {
	match PathBuf::from(filename).file_stem() {
		Some(file_stem) => file_stem.to_string_lossy().into_owned(),
		None => "".to_string()
	}
}

pub fn extension(filename: &str) -> String {
	match PathBuf::from(filename).extension() {
		Some(extension) => extension.to_string_lossy().into_owned(),
		None => "".to_string()
	}
}

pub fn filename(filename: &str) -> String {
	match PathBuf::from(filename).file_name() {
		Some(file_name) => file_name.to_string_lossy().into_owned(),
		None => "".to_string()
	}
}

pub fn exists(filepath: &str) -> bool {
	match File::open(filepath) {
		Err(_why) => false,
		Ok(_file) => true
	}
}
