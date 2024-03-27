use tauri::{ Manager, AppHandle, State };
use tauri::async_runtime::spawn;

use rfd::{ AsyncMessageDialog, MessageButtons, MessageDialogResult };

use crate::file::{ FileState, modify_file };
use crate::format::pray::Block;
use crate::format::agent_block::{ Agent, GameSupport, Description, Language };
use crate::format::egg_block::Egg;
use crate::format::gb_block::GardenBox;
use crate::dependency::check_dependencies_for_tag;

#[tauri::command]
pub fn select_tag(handle: AppHandle, file_state: State<FileState>, selected_tag: u32) {
	if let Some(tag) = file_state.tags.lock().unwrap().get(selected_tag as usize) {
		*file_state.selected_tag.lock().unwrap() = Some(selected_tag as usize);
		let checked_dependencies = check_dependencies_for_tag(tag, &mut file_state.dependencies.lock().unwrap());
		handle.emit("update_tag_info", &tag).unwrap();
		handle.emit("update_checked_dependencies", &checked_dependencies).unwrap();
		handle.emit("deselect_dependencies", ()).unwrap();
	}
}

#[tauri::command]
pub fn add_agent_tag(handle: AppHandle, file_state: State<FileState>) {
	let new_agent_tag = Block::Agent(Agent {
		name: "Agent".to_string(),
		game_support: GameSupport::DockingStation,
		descriptions: vec![Description::new(Language::English, String::new())],
		bioenergy: 0,
		web_label: String::new(),
		web_url: String::new(),
		animation_file: String::new(),
		animation_string: String::new(),
		sprite_first_image: 0,
		remove_script: String::new(),
		dependencies: Vec::new(),
	});
	add_tag(handle, file_state, new_agent_tag);
}

#[tauri::command]
pub fn add_egg_tag(handle: AppHandle, file_state: State<FileState>) {
	let new_egg_tag = Block::Egg(Egg {
		name: "Egg".to_string(),
		genetics_file: String::new(),
		genetics_file_mother: String::new(),
		genetics_file_father: String::new(),
		sprite_file_male: String::new(),
		sprite_file_female: String::new(),
		animation_string: String::new(),
		dependencies: Vec::new(),
	});
	add_tag(handle, file_state, new_egg_tag);
}

#[tauri::command]
pub fn add_gb_tag(handle: AppHandle, file_state: State<FileState>) {
	let new_gb_tag = Block::GardenBox(GardenBox {
		name: "Garden Box".to_string(),
		description: String::new(),
		author: String::new(),
		category: 1,
		animation_file: String::new(),
		sprite_first_image: 0,
		remove_script: String::new(),
		dependencies: Vec::new(),
	});
	add_tag(handle, file_state, new_gb_tag);
}

fn add_tag(handle: AppHandle, file_state: State<FileState>, new_tag: Block) {
	modify_file(&handle, true);
	let mut tags = file_state.tags.lock().unwrap();
	tags.push(new_tag);
	let selected_tag = tags.len() - 1;
	*file_state.selected_tag.lock().unwrap() = Some(selected_tag);
	handle.emit("update_tag_list", (selected_tag, tags.to_owned())).unwrap();
	handle.emit("update_checked_dependencies", Vec::<u32>::new()).unwrap();
	handle.emit("deselect_dependencies", ()).unwrap();
}

#[tauri::command]
pub fn duplicate_tag(handle: AppHandle, file_state: State<FileState>) {
	modify_file(&handle, true);
	let mut tags = file_state.tags.lock().unwrap();
	let selected_tag = *file_state.selected_tag.lock().unwrap();
	if let Some(selected_tag_index) = selected_tag {
		if let Some(original_tag) = tags.get(selected_tag_index) {
			let mut tag_copy = original_tag.clone();
			match tag_copy {
				Block::Agent(ref mut t) => { t.name = format!("{} Copy", t.name); }
				Block::Egg(ref mut t) => { t.name = format!("{} Copy", t.name); }
				Block::GardenBox(ref mut t) => { t.name = format!("{} Copy", t.name); }
				Block::Generic(ref mut t) => { t.name = format!("{} Copy", t.name); }
				_ => {}
			}
			tags.insert(selected_tag_index + 1, tag_copy);
			*file_state.selected_tag.lock().unwrap() = Some(selected_tag_index + 1);
			handle.emit("update_tag_list", (selected_tag_index + 1, tags.to_owned())).unwrap();
			handle.emit("deselect_dependencies", ()).unwrap();
		}
	}
}

#[tauri::command]
pub fn remove_tag(handle: AppHandle) {
	spawn(async move {
		let confirm_remove = AsyncMessageDialog::new()
			.set_title("Remove Tag")
			.set_description("Are you sure you want to remove this tag?")
			.set_buttons(MessageButtons::YesNo)
			.show()
			.await;

		if let MessageDialogResult::Yes = confirm_remove {
			let file_state: State<FileState> = handle.state();
			let selected_tag = *file_state.selected_tag.lock().unwrap();
			if let Some(selected_tag_index) = selected_tag {
				modify_file(&handle, true);
				let mut tags = file_state.tags.lock().unwrap();
				tags.remove(selected_tag_index);
				let selected_tag_index = if selected_tag_index >= 1 { selected_tag_index - 1 } else { 0 };
				*file_state.selected_tag.lock().unwrap() = if tags.is_empty() { None } else { Some(selected_tag_index) };
				if let Some(tag) = tags.get(selected_tag_index) {
					let checked_dependencies = check_dependencies_for_tag(tag, &mut file_state.dependencies.lock().unwrap());
					handle.emit("update_checked_dependencies", &checked_dependencies).unwrap();
					handle.emit("deselect_dependencies", ()).unwrap();
				}
				handle.emit("update_tag_list", (selected_tag_index, tags.to_owned())).unwrap();
			}
		}
	});
}
