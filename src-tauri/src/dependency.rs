use std::{
	fs,
	path::{ Path, PathBuf },
	ffi::OsStr,
	error::Error,
	collections::HashMap
};

use tauri::{ Manager, AppHandle, State };
use tauri::async_runtime::spawn;

use rfd::{ AsyncFileDialog, AsyncMessageDialog, MessageDialog, MessageButtons, MessageDialogResult };

use bytes::Bytes;

use crate::error_dialog;
use crate::file::{ FileState, modify_file };
use crate::format::pray::Block;
use crate::format::file_block::File;

static SUPPORTED_EXTENSIONS: [&str; 10] = ["cos", "wav", "mng", "c16", "s16", "blk", "gen", "gno", "att", "catalogue"];

#[tauri::command]
pub fn add_dependency(handle: AppHandle) {
	spawn(async move {
		let file_handle = AsyncFileDialog::new()
			.add_filter("Dependencies", &SUPPORTED_EXTENSIONS)
			.pick_file()
			.await;
		if let Some(file_handle) = file_handle {
			match add_dependency_from_path(&handle, file_handle.path().to_path_buf()) {
				Ok(()) => {},
				Err(why) => error_dialog(why.to_string())
			};
		}
	});
}

pub fn add_dependency_from_path(handle: &AppHandle, file_path: PathBuf) -> Result<(), Box<dyn Error>> {
	modify_file(handle, true);
	let extension = file_path.extension().unwrap_or(OsStr::new("")).to_str().unwrap_or("").to_ascii_lowercase();
	let name = file_path.file_stem().unwrap_or(OsStr::new("")).to_str().unwrap_or("");
	let dependency_name = format!("{}.{}", name, extension);
	if SUPPORTED_EXTENSIONS.contains(&extension.as_str()) {
		let file_state: State<FileState> = handle.state();
		let mut dependencies = file_state.dependencies.lock().unwrap();
		let dependency_names: Vec<String> = dependencies.clone().iter().map(|d| d.filename()).collect();
		if !dependency_names.contains(&dependency_name) {

			let bytes = fs::read(&file_path)?;

			let new_dependency = File {
				name: name.to_string(),
				extension: extension.to_string(),
				data: Bytes::copy_from_slice(&bytes),
				is_checked: true
			};

			dependencies.push(new_dependency);
			sort_dependencies(&mut dependencies);

			let mut tags = file_state.tags.lock().unwrap();
			let selected_tag = *file_state.selected_tag.lock().unwrap();
			if let Some(selected_tag_index) = selected_tag {
				if let Some(tag) = tags.get_mut(selected_tag_index) {
					match tag {
						Block::Agent(ref mut t) => { t.dependencies.push(dependency_name); }
						Block::Egg(ref mut t) => { t.dependencies.push(dependency_name); }
						Block::GardenBox(ref mut t) => { t.dependencies.push(dependency_name); }
						_ => {}
					}
					check_dependencies_for_tag(tag, &mut dependencies);
				}
			}

			handle.emit("update_dependency_list", dependencies.clone()).unwrap();
		}
		Ok(())

	} else {
		Err(format!("\"{}\" is not a supported file type", dependency_name).into())
	}
}

#[tauri::command]
pub fn extract_dependency(handle: AppHandle, selected_dependencies: Vec<u32>) {
	spawn(async move {
		let file_handle = AsyncFileDialog::new()
			.pick_folder()
			.await;
		if let Some(file_handle) = file_handle {
			let file_path = file_handle.path().to_path_buf();
			let file_state: State<FileState> = handle.state();
			let dependencies = file_state.dependencies.lock().unwrap();

			let mut ok_to_save = true;
			let mut num_files_to_overwrite: u32 = 0;
			let mut first_file_to_overwrite = String::new();
			let mut dependency_paths: HashMap<usize, PathBuf> = HashMap::new();

			for (i, dependency) in dependencies.iter().enumerate() {
				if selected_dependencies.contains(&(i as u32)) {
					let dependency_name = dependency.filename();
					let mut dependency_path = file_path.clone();
					dependency_path.push(dependency_name.clone());
					if let Ok(path_exists) = dependency_path.try_exists() {
						if path_exists {
							num_files_to_overwrite += 1;
							if first_file_to_overwrite.is_empty() {
								first_file_to_overwrite = dependency_name.clone();
							}
						}
					}
					dependency_paths.insert(i, dependency_path);
				}
			}

			if num_files_to_overwrite > 0 {
				let result = MessageDialog::new()
					.set_title(if num_files_to_overwrite == 1 { "Overwrite File" } else { "Overwrite Files" })
					.set_description(match num_files_to_overwrite {
						1 => format!("Ok to overwrite \"{}\"?", &first_file_to_overwrite),
						2 => format!("Ok to overwrite \"{}\" and 1 other file?", &first_file_to_overwrite),
						_ => format!("Ok to overwrite \"{}\" and {} other files?", &first_file_to_overwrite, num_files_to_overwrite - 1)
						})
					.set_buttons(MessageButtons::YesNo)
					.show();
				if let MessageDialogResult::No = result {
					ok_to_save = false;
				}
			}

			if ok_to_save {
				for (i, dependency) in dependencies.iter().enumerate() {
					if let Some(dependency_path) = dependency_paths.get(&i) {
						if let Err(why) = fs::write(dependency_path, &dependency.data) {
							error_dialog(why.to_string());
						}
					}
				}
				handle.emit("show_notification", if selected_dependencies.len() == 1 { "Dependency extracted" } else { "Dependencies extracted" }).unwrap();
			}
		}
	});
}

#[tauri::command]
pub fn reload_dependency(handle: AppHandle, file_state: State<FileState>, selected_dependencies: Vec<u32>) {
	let do_reload = |handle: AppHandle| -> Result<(), Box<dyn Error>> {
		let file_path = file_state.path.lock().unwrap().clone().unwrap_or(PathBuf::from(""));
		let root_path = file_path.parent().unwrap_or(Path::new(""));
		if !file_path.is_file() || !root_path.is_dir() {
			return Err("Agent file must be saved before reloading dependencies.".into());
		}
		let file_name_folder = file_path.file_stem().unwrap_or(OsStr::new(""));
		let dependencies = file_state.dependencies.lock().unwrap();
		let mut dependency_names: HashMap<usize, String> = HashMap::new();
		let mut dependency_paths: HashMap<usize, PathBuf> = HashMap::new();
		for (i, dependency) in dependencies.iter().enumerate() {
			if selected_dependencies.contains(&(i as u32)) {
				let dependency_name = dependency.filename();
				let long_path = root_path.join(file_name_folder).join(&dependency_name);
				let short_path = root_path.join(&dependency_name);
				if long_path.try_exists().unwrap_or(false) {
					dependency_paths.insert(i, long_path);
				} else if short_path.try_exists().unwrap_or(false) {
					dependency_paths.insert(i, short_path);
				}
				dependency_names.insert(i, dependency_name);
			}
		}

		spawn(async move {
			let confirm_reload = AsyncMessageDialog::new()
				.set_title(if selected_dependencies.len() == 1 { "Reload Dependency" } else { "Reload Dependencies" })
				.set_description(if selected_dependencies.len() == 1 {
						format!("Replace \"{}\" in agent with version from disk?",
							dependency_names.get(&(selected_dependencies[0] as usize)).unwrap_or(&"?".to_string()))
					} else {
						format!("Replace {} dependencies in agent with versions from disk?",
							selected_dependencies.len())
					})
				.set_buttons(MessageButtons::YesNo)
				.show()
				.await;

			if let MessageDialogResult::Yes = confirm_reload {
				modify_file(&handle, true);
				let file_state: State<FileState> = handle.state();
				let mut dependencies = file_state.dependencies.lock().unwrap();
				for (i, dependency) in dependencies.iter_mut().enumerate() {
					if let Some(dependency_path) = dependency_paths.get(&i) {
						if let Ok(data) = fs::read(dependency_path) {
							dependency.data = Bytes::from(data);
						}
					}
				}
				handle.emit("show_notification", if selected_dependencies.len() == 1 { "Dependency reloaded" } else { "Dependencies reloaded" }).unwrap();
			}
		});

		Ok(())
	};

	if let Err(why) = do_reload(handle) {
		error_dialog(why.to_string());
	}
}

#[tauri::command]
pub fn remove_dependency(handle: AppHandle, file_state: State<FileState>, selected_dependencies: Vec<u32>) {
	let dependencies = file_state.dependencies.lock().unwrap();
	let filename = dependencies.first().map(|dep| dep.name.clone());

	if let Some(filename) = filename {
		let handle = handle.clone();
		spawn(async move {
			let confirm_remove = AsyncMessageDialog::new()
				.set_title(if selected_dependencies.len() == 1 { "Remove Dependency" } else { "Remove Dependencies" })
				.set_description(if selected_dependencies.len() == 1 {
						format!("Remove {} from the agent? This won't delete the original file.", filename)
					} else {
						format!("Remove {} dependencies from the agent? This won't delete the original files.", selected_dependencies.len())
					})
				.set_buttons(MessageButtons::YesNo)
				.show()
				.await;

			if let MessageDialogResult::Yes = confirm_remove {
				modify_file(&handle, true);
				let file_state: State<FileState> = handle.state();
				let dependencies = file_state.dependencies.lock().unwrap().clone();
				let mut new_dependencies: Vec<File> = Vec::new();
				for (i, dependency) in dependencies.iter().enumerate() {
					if !selected_dependencies.contains(&(i as u32)) {
						new_dependencies.push(dependency.clone());
					}
				}

				remove_missing_dependencies(&file_state, &new_dependencies);

				handle.emit("update_dependency_list", new_dependencies.clone()).unwrap();
				*file_state.dependencies.lock().unwrap() = new_dependencies.clone();
			}
		});
	}
}

fn remove_missing_dependencies(file_state: &State<FileState>, dependencies: &[File]) {
	let mut tags = file_state.tags.lock().unwrap();
	let dependency_names: Vec<String> = dependencies.iter().map(|d| { d.filename() }).collect();
	for tag in tags.iter_mut() {
		match tag {
			Block::Agent(agent_tag) => {
				if !dependency_names.contains(&agent_tag.animation_file) {
					agent_tag.animation_file = String::new();
				}
			}
			Block::Egg(egg_tag) => {
				if !dependency_names.contains(&egg_tag.genetics_file) {
					egg_tag.genetics_file = String::new();
				}
				if !dependency_names.contains(&egg_tag.genetics_file_mother) {
					egg_tag.genetics_file_mother = String::new();
				}
				if !dependency_names.contains(&egg_tag.genetics_file_father) {
					egg_tag.genetics_file_father = String::new();
				}
				if !dependency_names.contains(&egg_tag.sprite_file_male) {
					egg_tag.sprite_file_male = String::new();
				}
				if !dependency_names.contains(&egg_tag.sprite_file_female) {
					egg_tag.sprite_file_female = String::new();
				}
			}
			Block::GardenBox(gb_tag) => {
				if !dependency_names.contains(&gb_tag.animation_file) {
					gb_tag.animation_file = String::new();
				}
			}
			_ => {}
		}
	}
}

#[tauri::command]
pub fn check_dependency(handle: AppHandle, file_state: State<FileState>, checked_dependencies: Vec<u32>) {
	modify_file(&handle, true);
	let dependencies = file_state.dependencies.lock().unwrap();
	let mut tags = file_state.tags.lock().unwrap();
	let selected_tag = *file_state.selected_tag.lock().unwrap();

	if let Some(selected_tag_index) = selected_tag {
		if let Some(tag) = tags.get_mut(selected_tag_index) {
			let mut tag_dependencies: Vec<String> = Vec::new();
			for (i, dependency) in dependencies.iter().enumerate() {
				if checked_dependencies.contains(&(i as u32)) {
					tag_dependencies.push(dependency.filename());
				}
			}
			match tag {
				Block::Agent(ref mut t) => { t.dependencies = tag_dependencies; }
				Block::Egg(ref mut t) => { t.dependencies = tag_dependencies; }
				Block::GardenBox(ref mut t) => { t.dependencies = tag_dependencies; }
				_ => {}
			}
		}
	}
}

pub fn check_dependencies_for_tag(tag: &Block, dependencies: &mut [File]) -> Vec<u32> {
	let empty_deps = Vec::new();
	let tag_dependencies = match tag {
		Block::Agent(t) => &t.dependencies,
		Block::Egg(t) => &t.dependencies,
		Block::GardenBox(t) => &t.dependencies,
		_ => &empty_deps
	};

	let mut checked_dependencies: Vec<u32> = Vec::new();

	for dependency in dependencies.iter_mut() {
		dependency.is_checked = false;
	}

	for tag_dependency in tag_dependencies {
		for (i, dependency) in dependencies.iter_mut().enumerate() {
			if *tag_dependency == dependency.filename() {
				dependency.is_checked = true;
				checked_dependencies.push(i as u32);
			}
		}
	}

	checked_dependencies
}

pub fn sort_dependencies(dependencies: &mut [File]) {
	dependencies.sort_by_key(|d| (match d.extension.as_str() {
		"cos" => 0,
		"gen" => 1,
		"gno" => 2,
		"catalogue" => 3,
		"blk" => 4,
		"c16" => 5,
		"s16" => 6,
		"mng" => 7,
		"wav" => 8,
		"att" => 9,
		_ => 10
	}, d.name.clone()));
}
