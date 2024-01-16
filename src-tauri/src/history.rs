use std::sync::Mutex;
use std::path::PathBuf;

use tauri::{ Manager, AppHandle, State };
use tauri::menu::MenuItemKind;

use crate::update_title;
use crate::file::{ FileState, modify_file };
use crate::tag::select_tag;
use crate::format::pray::Block;
use crate::format::file_block::File;

pub struct HistoryState {
	pub undo_stack: Mutex<Vec<HistoryItem>>,
	pub redo_stack: Mutex<Vec<HistoryItem>>
}

pub struct HistoryItem {
	pub path: Option<PathBuf>,
	pub is_modified: bool,
	pub dependencies: Vec<File>,
	pub tags: Vec<Block>,
	pub selected_tag: Option<usize>
}

pub fn reset_history(handle: &AppHandle) {
	let history_state: State<HistoryState> = handle.state();
	history_state.undo_stack.lock().unwrap().clear();
	history_state.redo_stack.lock().unwrap().clear();
}

pub fn add_history_state(handle: &AppHandle) {
	let history_state: State<HistoryState> = handle.state();
	let current_state = get_current_state(handle);
	history_state.undo_stack.lock().unwrap().push(current_state);
	history_state.redo_stack.lock().unwrap().clear();
	update_history_menu_items(handle);
}

#[tauri::command]
pub fn undo(handle: AppHandle) {
	let history_state: State<HistoryState> = handle.state();
	let history_item = history_state.undo_stack.lock().unwrap().pop();
	if let Some(history_item) = history_item {
		let current_state = get_current_state(&handle);
		history_state.redo_stack.lock().unwrap().push(current_state);
		set_current_state(&handle, &history_item);
	}
	modify_file(&handle, false);
}

#[tauri::command]
pub fn redo(handle: AppHandle) {
	let history_state: State<HistoryState> = handle.state();
	let history_item = history_state.redo_stack.lock().unwrap().pop();
	if let Some(history_item) = history_item {
		let current_state = get_current_state(&handle);
		history_state.undo_stack.lock().unwrap().push(current_state);
		set_current_state(&handle, &history_item);
	}
	modify_file(&handle, false);
}

fn get_current_state(handle: &AppHandle) -> HistoryItem {
	let file_state: State<FileState> = handle.state();
	let path = file_state.path.lock().unwrap().clone();
	let is_modified = *file_state.is_modified.lock().unwrap();
	let dependencies = file_state.dependencies.lock().unwrap().clone();
	let tags = file_state.tags.lock().unwrap().clone();
	let selected_tag = *file_state.selected_tag.lock().unwrap();
	HistoryItem { path, is_modified, dependencies, tags, selected_tag }
}

fn set_current_state(handle: &AppHandle, history_item: &HistoryItem) {
	let file_state: State<FileState> = handle.state();

	*file_state.path.lock().unwrap() = history_item.path.clone();
	*file_state.is_modified.lock().unwrap() = history_item.is_modified;
	*file_state.dependencies.lock().unwrap() = history_item.dependencies.clone();
	*file_state.tags.lock().unwrap() = history_item.tags.clone();
	*file_state.selected_tag.lock().unwrap() = history_item.selected_tag;

	handle.emit("update_dependency_list", &history_item.dependencies).unwrap();
	handle.emit("update_tag_list", (history_item.selected_tag.unwrap_or(0), &history_item.tags)).unwrap();
	select_tag(handle.clone(), file_state, history_item.selected_tag.unwrap_or(0) as u32);

	update_title(handle);
	update_history_menu_items(handle);
}

fn update_history_menu_items(handle: &AppHandle) {
	let history_state: State<HistoryState> = handle.state();
	if let Some(menu) = handle.menu() {
		if let Some(MenuItemKind::Submenu(edit_menu)) = menu.get("edit") {
			if let Some(MenuItemKind::MenuItem(menu_item)) = edit_menu.get("undo") {
				let enable_undo = history_state.undo_stack.lock().unwrap().len() > 0;
				menu_item.set_enabled(enable_undo).unwrap();
				handle.emit("enable_undo_button", enable_undo).unwrap();
			};
			if let Some(MenuItemKind::MenuItem(menu_item)) = edit_menu.get("redo") {
				let enable_redo = history_state.redo_stack.lock().unwrap().len() > 0;
				menu_item.set_enabled(enable_redo).unwrap();
				handle.emit("enable_redo_button", enable_redo).unwrap();
			};
		}
	}
}
