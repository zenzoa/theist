use std::{
	fs,
	error::Error,
	path::{ Path, PathBuf },
	sync::Mutex,
	ffi::OsStr,
	collections::HashMap
};

use tauri::{ AppHandle, State, Manager, Emitter };

use rfd::{ FileDialog, MessageDialog, MessageButtons, MessageDialogResult };

use image::RgbaImage;

use crate::error_dialog;
use crate::update_title;
use crate::format::pray::{ Block, encode, decode };
use crate::format::file_block::File;
use crate::history::{
	reset_history,
	add_history_state
};
use crate::dependency::{
	sort_dependencies,
	check_dependencies_for_tag,
	add_dependency_from_path
};

pub struct FileState {
	pub path: Mutex<Option<PathBuf>>,
	pub is_modified: Mutex<bool>,
	pub dependencies: Mutex<Vec<File>>,
	pub tags: Mutex<Vec<Block>>,
	pub selected_tag: Mutex<Option<usize>>,
	pub image_cache: Mutex<ImageCache>
}

pub struct FileModifiedCallback {
	pub func: fn(AppHandle, PathBuf)
}

pub struct ImageCache(HashMap<String, Vec<RgbaImage>>);

impl ImageCache {
	pub fn new() -> Self {
		Self(HashMap::<String, Vec<RgbaImage>>::new())
	}
	pub fn get(&self, key: &str) -> Option<&Vec<RgbaImage>> {
		self.0.get(key)
	}
	pub fn insert(&mut self, key: String, value: Vec<RgbaImage>) -> Option<Vec<RgbaImage>> {
		self.0.insert(key, value)
	}
}

pub fn check_file_modified(handle: AppHandle, path: PathBuf, callback: FileModifiedCallback) {
	let file_state: State<FileState> = handle.state();
	if *file_state.is_modified.lock().unwrap() {
		let confirm_reload = MessageDialog::new()
			.set_title("File modified")
			.set_description("Do you want to continue anyway and lose any unsaved work?")
			.set_buttons(MessageButtons::YesNo)
			.show();
		if let MessageDialogResult::Yes = confirm_reload {
			(callback.func)(handle, path);
		}
	} else {
		(callback.func)(handle, path);
	}
}

fn reset_file_modified(handle: &AppHandle) {
	let file_state: State<FileState> = handle.state();
	*file_state.is_modified.lock().unwrap() = false;

	update_title(handle);
}

pub fn modify_file(handle: &AppHandle, add_to_history: bool) {
	if add_to_history {
		add_history_state(handle);
	}

	let file_state: State<FileState> = handle.state();
	*file_state.is_modified.lock().unwrap() = true;

	update_title(handle);
}

pub fn create_file_dialog(handle: &AppHandle) -> FileDialog {
	let mut file_dialog = FileDialog::new();

	let file_state: State<FileState> = handle.state();
	if let Some(file_path) = file_state.path.lock().unwrap().clone() {
		if let Some(parent_dir) = file_path.parent() {
			file_dialog = file_dialog.set_directory(parent_dir);
		}
	}

	file_dialog
}

#[tauri::command]
pub fn new_file(handle: AppHandle) {
	check_file_modified(handle, PathBuf::new(), FileModifiedCallback { func: |handle, _| {
		let file_state: State<FileState> = handle.state();
		*file_state.path.lock().unwrap() = None;
		*file_state.dependencies.lock().unwrap() = Vec::new();
		*file_state.tags.lock().unwrap() = Vec::new();
		*file_state.selected_tag.lock().unwrap() = None;
		reset_history(&handle);
		reset_file_modified(&handle);
		handle.emit("update_tag_list", (0, Vec::<String>::new())).unwrap();
		handle.emit("update_dependency_list", Vec::<String>::new()).unwrap();
	}});
}

#[tauri::command]
pub fn open_file(handle: AppHandle) {
	check_file_modified(handle, PathBuf::new(), FileModifiedCallback { func: |handle, _| {
		open_file_dialog(&handle);
	}});
}

pub fn open_file_dialog(handle: &AppHandle) {
	let handle = handle.clone();
	let file_handle = create_file_dialog(&handle)
		.add_filter("Agents", &["agent", "agents"])
		.pick_file();
	if let Some(file_handle) = file_handle {
		match open_file_from_path(&handle, &file_handle.as_path().to_path_buf()) {
			Ok(()) => {},
			Err(why) => error_dialog(why.to_string())
		};
	}
}

pub fn open_file_from_path(handle: &AppHandle, file_path: &PathBuf) -> Result<(), Box<dyn Error>> {
	handle.emit("show_spinner", ()).unwrap();

	let bytes = fs::read(file_path)?;
	let blocks = decode(&bytes)?;

	let file_state: State<FileState> = handle.state();
	*file_state.path.lock().unwrap() = Some(file_path.clone());

	reset_history(handle);
	reset_file_modified(handle);

	let mut dependencies: Vec<File> = Vec::new();
	let mut tags: Vec<Block> = Vec::new();

	for block in blocks {
		match block {
			Block::File(file) => { dependencies.push(file); }
			_ => { tags.push(block); }
		}
	}

	match tags.first() {
		Some(tag) => {
			*file_state.selected_tag.lock().unwrap() = Some(0);
			check_dependencies_for_tag(tag, &mut dependencies);
		}

		_ => {
			*file_state.selected_tag.lock().unwrap() = None;
		}
	}

	sort_dependencies(&mut dependencies);

	handle.emit("update_dependency_list", &dependencies).unwrap();
	handle.emit("update_tag_list", (0, &tags)).unwrap();

	update_title(handle);

	*file_state.dependencies.lock().unwrap() = dependencies;
	*file_state.tags.lock().unwrap() = tags;

	handle.emit("hide_spinner", ()).unwrap();

	Ok(())
}

#[tauri::command]
pub fn save_file(handle: AppHandle) {
	let file_state: State<FileState> = handle.state();
	let is_modified = *file_state.is_modified.lock().unwrap();
	if is_modified {
		let path = file_state.path.lock().unwrap().clone();
		if let Some(file_path) = path {
			if file_path.exists() {
				save_file_to_path(handle, &file_path);
				return;
			}
		}
		save_file_as(handle);
	}
}

#[tauri::command]
pub fn save_file_as(handle: AppHandle) {
	let file_handle = create_file_dialog(&handle)
		.add_filter("Agents", &["agent", "agents"])
		.save_file();
	if let Some(file_handle) = file_handle {
		save_file_to_path(handle, file_handle.as_path());
	}
}

fn save_file_to_path(handle: AppHandle, file_path: &Path) {
	let file_state: State<FileState> = handle.state();
	*file_state.path.lock().unwrap() = Some(PathBuf::from(file_path));
	let tags = file_state.tags.lock().unwrap().clone();
	let dependencies = file_state.dependencies.lock().unwrap().clone();
	match encode(&tags, &dependencies) {
		Ok(bytes) => {
			match fs::write(file_path, &bytes) {
				Ok(()) => {
					reset_file_modified(&handle);
					handle.emit("show_notification", "Agent file saved").unwrap();
				}
				Err(why) => error_dialog(why.to_string())
			}
		},
		Err(why) => error_dialog(why.to_string())
	}
}

pub fn drop_file(handle: &AppHandle, paths: &Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
	if let Some(first_path) = paths.first() {
		let first_extension = first_path.extension().unwrap_or(OsStr::new("")).to_ascii_lowercase();
		if first_extension == "agent" || first_extension == "agents" {
			check_file_modified(handle.clone(), first_path.clone(), FileModifiedCallback { func: |handle, path| {
				if let Err(why) = open_file_from_path(&handle, &path) {
					error_dialog(why.to_string());
				}
			}});
		} else {
			for path in paths {
				add_dependency_from_path(handle, path.clone())?;
			}
		}
	}
	Ok(())
}
