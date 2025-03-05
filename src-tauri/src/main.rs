// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io::Cursor;
use std::sync::Mutex;
use std::path::PathBuf;

use tauri::{
	Builder,
	AppHandle,
	WindowEvent,
	DragDropEvent,
	Manager,
	State,
	Emitter
};

use tauri::menu::{
	Menu,
	Submenu,
	MenuItem,
	CheckMenuItem,
	PredefinedMenuItem,
	MenuId
};

use rfd::{ MessageDialog, MessageButtons };

mod file;
mod format;
mod sprite;
mod tag;
mod tag_info;
mod dependency;
mod history;
mod config;

use file::{ FileState, FileModifiedCallback, check_file_modified };
use history::HistoryState;
use config::ConfigState;

fn main() {

	Builder::default()

		.on_window_event(|window, event| {
			match event {
				WindowEvent::DragDrop(DragDropEvent::Drop{ paths, position: _ }) => {
					if !paths.is_empty() {
						if let Err(why) = file::drop_file(window.app_handle(), paths) {
							error_dialog(why.to_string());
						}
					}
				},
				WindowEvent::CloseRequested { api, .. } => {
					api.prevent_close();
					try_quit(window.app_handle().clone());
				}
				_ => {}
			}
		})

		.menu(|handle| {
			Menu::with_id_and_items(handle, "main", &[

				&Submenu::with_id_and_items(handle, "file", "File", true, &[
					&MenuItem::with_id(handle, "new", "New", true, Some("CmdOrCtrl+N"))?,
					&MenuItem::with_id(handle, "open", "Open", true, Some("CmdOrCtrl+O"))?,
					&PredefinedMenuItem::separator(handle)?,
					&MenuItem::with_id(handle, "save", "Save", true, Some("CmdOrCtrl+S"))?,
					&MenuItem::with_id(handle, "save_as", "Save As", true, Some("CmdOrCtrl+Shift+S"))?,
					&PredefinedMenuItem::separator(handle)?,
					&MenuItem::with_id(handle, "quit", "Quit", true, Some("CmdOrCtrl+Q"))?,
				])?,

				&Submenu::with_id_and_items(handle, "edit", "Edit", true, &[
					&MenuItem::with_id(handle, "undo", "Undo", true, Some("CmdOrCtrl+Z"))?,
					&MenuItem::with_id(handle, "redo", "Redo", true, Some("CmdOrCtrl+Shift+Z"))?,
					&PredefinedMenuItem::separator(handle)?,
					&MenuItem::with_id(handle, "add_tag", "Add Tag", true, Some("CmdOrCtrl+Shift+N"))?,
				])?,

				&Submenu::with_id_and_items(handle, "view", "View", true, &[
					&Submenu::with_id_and_items(handle, "theme", "Theme", true, &[
						&CheckMenuItem::with_id(handle, "theme_dark", "Dark", true, true, None::<&str>)?,
						&CheckMenuItem::with_id(handle, "theme_light", "Light", true, false, None::<&str>)?,
						&CheckMenuItem::with_id(handle, "theme_purple", "Purple", true, false, None::<&str>)?,
					])?,
				])?,

				&Submenu::with_id_and_items(handle, "help", "Help", true, &[
					&MenuItem::with_id(handle, "about", "About", true, None::<&str>)?,
				])?,

			])
		})

		.setup(|app| {
			app.on_menu_event(|handle, event| {
				let MenuId(id) = event.id();
				let handle = handle.clone();
				match id.as_str() {

					"new" => file::new_file(handle),
					"open" => file::open_file(handle),
					"save" => file::save_file(handle),
					"save_as" => file::save_file_as(handle),
					"quit" => try_quit(handle),

					"undo" => history::undo(handle),
					"redo" => history::redo(handle),

					"add_tag" => handle.emit("show_add_tag_dialog", ()).unwrap(),

					"theme_dark" => config::set_theme(&handle, "dark", false),
					"theme_light" => config::set_theme(&handle, "light", false),
					"theme_purple" => config::set_theme(&handle, "purple", false),

					"about" => handle.emit("show_about_dialog", "").unwrap(),

					_ => {}
				}
			});
			Ok(())
		})

		.manage(FileState {
			path: Mutex::new(None),
			is_modified: Mutex::new(false),
			dependencies: Mutex::new(Vec::new()),
			tags: Mutex::new(Vec::new()),
			selected_tag: Mutex::new(None),
			image_cache: Mutex::new(file::ImageCache::new())
		})

		.manage(HistoryState {
			undo_stack: Mutex::new(Vec::new()),
			redo_stack: Mutex::new(Vec::new()),
		})

		.manage(ConfigState {
			theme: Mutex::new(config::Theme::Dark),
		})

		.invoke_handler(tauri::generate_handler![
			try_quit,

			file::new_file,
			file::open_file,
			file::save_file,
			file::save_file_as,

			history::undo,
			history::redo,

			tag::select_tag,
			tag::add_agent_tag,
			tag::add_egg_tag,
			tag::add_gb_tag,
			tag::duplicate_tag,
			tag::remove_tag,

			tag_info::update_prop_str,
			tag_info::update_prop_int,

			tag_info::generate_remove_script,

			tag_info::update_description_language,
			tag_info::update_description_text,
			tag_info::remove_description,
			tag_info::add_description,

			dependency::add_dependency,
			dependency::extract_dependency,
			dependency::reload_dependency,
			dependency::remove_dependency,
			dependency::check_dependency,
			dependency::select_dependency,
			dependency::deselect_dependency,
			dependency::export_dependency,
		])

		.on_page_load(|window, _| {
			config::load_config_file(window.app_handle());
		})

		.register_uri_scheme_protocol("getimage", |context, request| {
			let handle = context.app_handle();

			let not_found = http::Response::builder().body(Vec::new()).unwrap();

			let uri = request.uri().path();
			let uri_parts:Vec<&str> = uri.split('/').collect();
			if let Some(filename) = uri_parts.get(2) {
				if let Some(frame_index_str) = uri_parts.get(3) {
					let frame_index_result: Result<usize, _> = frame_index_str.parse();
					if let Ok(frame_index) = frame_index_result {
						let file_state: State<file::FileState> = handle.state();
						let image_cache = file_state.image_cache.lock().unwrap();
						if let Some(frames) = image_cache.get(filename) {
							if let Some(frame) = frames.get(frame_index) {
								let mut data = Cursor::new(Vec::new());
								if let Ok(()) = frame.write_to(&mut data, image::ImageFormat::Png) {
									return http::Response::builder()
										.header("Content-Type", "image/png")
										.body(data.into_inner())
										.unwrap()
								}
							}
						}
					}
				}
			}

			not_found
		})

		.run(tauri::generate_context!())

		.expect("error while running tauri application");

}

#[tauri::command]
fn try_quit(handle: AppHandle) {
	check_file_modified(handle, PathBuf::new(), FileModifiedCallback { func: |handle, _| {
		if let Some(window) = handle.get_webview_window("main") {
			window.destroy().unwrap();
		};
	}});
}

pub fn update_title(handle: &AppHandle) {
	if let Some(window) = handle.get_webview_window("main") {
		let file_state: State<FileState> = handle.state();
		let is_modified = *file_state.is_modified.lock().unwrap();
		let modified_indicator = if is_modified { "*" } else { "" };
		let path = file_state.path.lock().unwrap().clone();
		if let Some(path) = path {
			if let Some(file_stem) = path.file_stem() {
				window.set_title(&format!("{}{} - Theist", &modified_indicator, file_stem.to_str().unwrap_or("Untitled"))).unwrap();
				return;
			}
		}
		if is_modified {
			window.set_title("*Untitled - Theist").unwrap();
		} else {
			window.set_title("Theist").unwrap();
		}
	}
}

pub fn error_dialog(error_message: String) {
	MessageDialog::new()
		.set_title("Error")
		.set_description(error_message)
		.set_buttons(MessageButtons::Ok)
		.show();
}
