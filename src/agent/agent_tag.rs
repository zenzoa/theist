use crate::agent::file::CreaturesFile;

#[derive(Clone)]
pub struct AgentTag {
	pub name: String,
	pub version: String,

	pub description: String,
	pub supported_game: SupportedGame,
	pub remove_script: RemoveScript,
	pub preview: Preview,

	pub remove_script_backup: RemoveScript,
	pub preview_backup: Preview,

	pub scripts: Vec<usize>,
	pub sprites: Vec<usize>,
	pub sounds: Vec<usize>,
	pub catalogues: Vec<usize>,

	pub use_all_files: bool
}

#[derive(Clone, PartialEq)]
pub enum SupportedGame {
	C3,
	DS,
	C3DS
}

#[derive(Clone, PartialEq)]
pub enum RemoveScript {
	None,
	Auto,
	Manual(String)
}

#[derive(Clone, PartialEq)]
pub enum Preview {
	None,
	Auto,
	Manual { sprite: usize, animation: String }
}

impl AgentTag {
	pub fn new() -> AgentTag {
		AgentTag {
			name: "Untitled Agent".to_string(),
			version: "".to_string(),

			description: "".to_string(),
			supported_game: SupportedGame::C3DS,
			remove_script: RemoveScript::None,
			preview: Preview::None,

			remove_script_backup: RemoveScript::None,
			preview_backup: Preview::None,

			scripts: Vec::new(),
			sprites: Vec::new(),
			sounds: Vec::new(),
			catalogues: Vec::new(),

			use_all_files: false
		}
	}

	pub fn has_sprite(&self, file_index: usize) -> bool {
		for sprite_index in &self.sprites {
			if sprite_index == &file_index {
				return true;
			}
		}
		false
	}

	pub fn get_first_sprite(&self, files: &[CreaturesFile]) -> Option<usize> {
		for sprite_index in &self.sprites {
			if let Some(sprite_file) = files.get(*sprite_index) {
				if &sprite_file.get_extension() != "blk" {
					return Some(*sprite_index);
				}
			}
		}
		None
	}
}
