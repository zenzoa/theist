use std::str;

use regex::Regex;

use tauri::{ Manager, AppHandle, State };
use tauri::async_runtime::spawn;

use rfd::{ AsyncMessageDialog, MessageButtons, MessageDialogResult };

use crate::file::{ FileState, modify_file };
use crate::format::pray::Block;
use crate::format::agent_block::{ GameSupport, Language, Description };

#[tauri::command]
pub fn update_prop_str(handle: AppHandle, file_state: State<FileState>, prop: &str, value: &str) {
	modify_file(&handle, true);
	if let Some(selected_tag) = *file_state.selected_tag.lock().unwrap() {
		if let Some(tag) = file_state.tags.lock().unwrap().get_mut(selected_tag) {
			match tag {
				Block::Agent(agent_tag) => {
					match prop {
						"game_support" => {
							match value {
								"Creatures3" => {
									agent_tag.game_support = GameSupport::Creatures3;
									handle.emit("update_tag_info", &tag).unwrap();
								}
								"DockingStation" => {
									agent_tag.game_support = GameSupport::DockingStation;
									handle.emit("update_tag_info", &tag).unwrap();
								}
								_ => {}
							}
						}
						"name" => {
							agent_tag.name = value.to_string();
						}
						"web_label" => {
							agent_tag.web_label = value.to_string();
						}
						"web_url" => {
							agent_tag.web_url = value.to_string();
						}
						"animation_file" => {
							agent_tag.animation_file = value.to_string();
						}
						"animation_string" => {
							agent_tag.animation_string = value.to_string();
						}
						"remove_script" => {
							agent_tag.remove_script = value.to_string();
						}
						_ => {}
					}
				}
				Block::Egg(egg_tag) => {
					match prop {
						"name" => {
							egg_tag.name = value.to_string();
						}
						"genetics_file" => {
							egg_tag.genetics_file = value.to_string();
						}
						"genetics_file_mother" => {
							egg_tag.genetics_file_mother = value.to_string();
						}
						"genetics_file_father" => {
							egg_tag.genetics_file_father = value.to_string();
						}
						"sprite_file_male" => {
							egg_tag.sprite_file_male = value.to_string();
						}
						"sprite_file_female" => {
							egg_tag.sprite_file_female = value.to_string();
						}
						"animation_string" => {
							egg_tag.animation_string = value.to_string();
						}
						_ => {}
					}
				}
				Block::GardenBox(gb_tag) => {
					match prop {
						"name" => {
							gb_tag.name = value.to_string();
						}
						"description" => {
							gb_tag.description = value.to_string();
						}
						"author" => {
							gb_tag.author = value.to_string();
						}
						"animation_file" => {
							gb_tag.animation_file = value.to_string();
						}
						_ => {}
					}
				}
				_ => {}
			}
		}
	}
}

#[tauri::command]
pub fn update_prop_int(handle: AppHandle, file_state: State<FileState>, prop: &str, value: u32) {
	modify_file(&handle, true);
	if let Some(selected_tag) = *file_state.selected_tag.lock().unwrap() {
		if let Some(tag) = file_state.tags.lock().unwrap().get_mut(selected_tag) {
			match tag {
				Block::Agent(agent_tag) => {
					match prop {
						"bioenergy" => {
							agent_tag.bioenergy = value;
						}
						"sprite_first_image" => {
							agent_tag.sprite_first_image = value;
						}
						_ => {}
					}
				}
				Block::GardenBox(gb_tag) => {
					match prop {
						"category" => {
							gb_tag.category = value;
						}
						"sprite_first_image" => {
							gb_tag.sprite_first_image = value;
						}
						_ => {}
					}
				}
				_ => {}
			}
		}
	}
}

#[tauri::command]
pub fn generate_remove_script(handle: AppHandle, file_state: State<FileState>) {
	let mut script_file_name = String::new();
	let mut remove_script = String::new();

	let remove_script_pattern = Regex::new(r"[\n\r]rscr[\n\r]([\s\S]*)").unwrap();
	let remove_comments_pattern = Regex::new(r"(?m)^\s+\*.*$").unwrap();
	let remove_newlines_pattern = Regex::new(r"\s+").unwrap();

	if let Some(selected_tag) = *file_state.selected_tag.lock().unwrap() {
		if let Some(Block::Agent(agent_tag)) = file_state.tags.lock().unwrap().get(selected_tag) {
			for dependency in file_state.dependencies.lock().unwrap().iter() {
				if &dependency.extension == "cos" && agent_tag.dependencies.contains(&dependency.filename()) {
					if let Ok(script) = str::from_utf8(&dependency.data) {
						if let Some(captures) = remove_script_pattern.captures(script) {
							if let Some(raw_remove_script) = captures.get(1) {
								remove_script = remove_comments_pattern.replace_all(raw_remove_script.as_str(), " ").to_string();
								remove_script = remove_newlines_pattern.replace_all(remove_script.as_str(), " ").trim().to_string();
								script_file_name = dependency.filename();
							}
						}
					}
				}
			}
		}
	}

	if remove_script.is_empty() {
		spawn(async {
			AsyncMessageDialog::new()
				.set_title("Remove Script Not Found")
				.set_description("No remove script found in any of this tag's COS files.")
				.show()
				.await;
		});

	} else {
		let handle = handle.clone();
		spawn(async move {
			let confirm_overwrite = AsyncMessageDialog::new()
				.set_title("Overwrite Remove Script")
				.set_description(format!("Replace current remove script with the one in {}?", script_file_name))
				.set_buttons(MessageButtons::YesNo)
				.show()
				.await;

			if let MessageDialogResult::Yes = confirm_overwrite {
				modify_file(&handle, true);
				let file_state: State<FileState> = handle.state();
				let selected_tag = *file_state.selected_tag.lock().unwrap();
				if let Some(selected_tag) = selected_tag {
					if let Some(tag) = file_state.tags.lock().unwrap().get_mut(selected_tag) {
						if let Block::Agent(agent_tag) = tag {
							agent_tag.remove_script = remove_script;
							handle.emit("update_tag_info", &tag).unwrap();
						}
					}
				}
			}
		});
	}
}

#[tauri::command]
pub fn update_description_language(handle: AppHandle, file_state: State<FileState>, index: u32, value: &str) {
	modify_file(&handle, true);
	if let Some(selected_tag) = *file_state.selected_tag.lock().unwrap() {
		if let Some(Block::Agent(tag)) = file_state.tags.lock().unwrap().get_mut(selected_tag) {
			if let Some(description) = tag.descriptions.get_mut(index as usize) {
				match value {
					"English" => { description.language = Language::English; }
					"German" => { description.language = Language::German; }
					"Spanish" => { description.language = Language::Spanish; }
					"French" => { description.language = Language::French; }
					"Italian" => { description.language = Language::Italian; }
					"Dutch" => { description.language = Language::Dutch; }
					_ => {}
				}
			}
		}
	}
}

#[tauri::command]
pub fn update_description_text(handle: AppHandle, file_state: State<FileState>, index: usize, value: &str) {
	modify_file(&handle, true);
	if let Some(selected_tag) = *file_state.selected_tag.lock().unwrap() {
		if let Some(Block::Agent(tag)) = file_state.tags.lock().unwrap().get_mut(selected_tag) {
			if let Some(description) = tag.descriptions.get_mut(index) {
				description.text = value.to_string();
			}
		}
	}
}

#[tauri::command]
pub fn remove_description(handle: AppHandle, index: usize) {
	spawn(async move {
		let confirm_remove = AsyncMessageDialog::new()
			.set_title("Remove Description")
			.set_description("Are you sure you want to remove this description?")
			.set_buttons(MessageButtons::YesNo)
			.show()
			.await;

		if let MessageDialogResult::Yes = confirm_remove {
			modify_file(&handle, true);
			let file_state: State<FileState> = handle.state();
			let selected_tag = *file_state.selected_tag.lock().unwrap();
			if let Some(selected_tag) = selected_tag {
				if let Some(tag) = file_state.tags.lock().unwrap().get_mut(selected_tag) {
					if let Block::Agent(agent_tag) = tag {
						agent_tag.descriptions.remove(index);
						handle.emit("update_tag_info", &tag).unwrap();
					}
				}
			}
		}
	});
}

#[tauri::command]
pub fn add_description(handle: AppHandle, file_state: State<FileState>) {
	modify_file(&handle, true);
	if let Some(selected_tag) = *file_state.selected_tag.lock().unwrap() {
		if let Some(tag) = file_state.tags.lock().unwrap().get_mut(selected_tag) {
			if let Block::Agent(agent_tag) = tag {
				let last_language = match agent_tag.descriptions.last() {
					Some(description) => description.language.clone(),
					None => Language::English
				};
				let new_language = match last_language {
					Language::English => Language::Spanish,
					Language::Spanish => Language::French,
					Language::French => Language::German,
					Language::German => Language::Italian,
					Language::Italian => Language::Dutch,
					Language::Dutch => Language::Spanish,
				};
				agent_tag.descriptions.push(Description {
					language: new_language,
					text: String::new()
				});
				handle.emit("update_tag_info", &tag).unwrap();
			}
		}
	}
}
