use std::sync::Mutex;
use std::{ fs, fmt };

use tauri::{
	AppHandle,
	Manager,
	State
};
use tauri::menu::MenuItemKind;

pub struct ConfigState {
	pub theme: Mutex<Theme>
}

#[derive(Clone, serde::Serialize)]
pub struct ConfigInfo {
	pub theme: String
}

#[derive(Clone, Debug)]
pub enum Theme {
	Dark,
	Light,
	Purple
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Theme::Dark => { write!(f, "dark") }
			Theme::Light => { write!(f, "light") }
			Theme::Purple => { write!(f, "purple") }
		}
    }
}

pub fn load_config_file(handle: &AppHandle) {
	if let Ok(config_dir) = handle.path().config_dir() {
		let config_file_path = config_dir.join("theist.conf");
		if let Ok(config_contents) = fs::read_to_string(config_file_path) {
			let lines: Vec<&str> = config_contents.split('\n').collect();
			for line in lines.iter() {
				let parts: Vec<&str> = line.split(':').collect();
				if let Some(key) = parts.first() {
					if let Some(value) = parts.get(1) {
						if key.trim() == "theme"{
							set_theme(handle, value.trim(), true);
						}
					}
				}
			}
		}
	}
}

pub fn save_config_file(handle: &AppHandle) {
	let config_state: State<ConfigState> = handle.state();
	if let Ok(config_dir) = handle.path().config_dir() {
		let config_file_path = config_dir.join("theist.conf");
		if let Ok(()) = fs::create_dir_all(config_dir) {
			fs::write(config_file_path, format!(
				"theme: {}",
				config_state.theme.lock().unwrap(),
			)).unwrap();
		}
	}
}

pub fn set_theme(handle: &AppHandle, new_theme: &str, init: bool) {
	handle.emit("set_theme", new_theme).unwrap();

	if let Some(menu) = handle.menu() {
		if let Some(MenuItemKind::Submenu(view_menu)) = menu.get("view") {
			if let Some(MenuItemKind::Submenu(theme_menu)) = view_menu.get("theme") {
				if let Some(MenuItemKind::Check(menu_item)) = theme_menu.get("theme_dark") {
					menu_item.set_checked(new_theme == "dark").unwrap();
				};
				if let Some(MenuItemKind::Check(menu_item)) = theme_menu.get("theme_light") {
					menu_item.set_checked(new_theme == "light").unwrap();
				};
				if let Some(MenuItemKind::Check(menu_item)) = theme_menu.get("theme_purple") {
					menu_item.set_checked(new_theme == "purple").unwrap();
				};
			}
		}
	}

	let config_state: State<ConfigState> = handle.state();
	*config_state.theme.lock().unwrap() = match new_theme {
		"light" => Theme::Light,
		"purple" => Theme::Purple,
		_ => Theme::Dark
	};

	if !init { save_config_file(handle); }
}
