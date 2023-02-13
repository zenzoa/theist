use crate::agent::file::CreaturesFile;

#[derive(Clone)]
pub struct EggTag {
	pub name: String,
	pub version: String,

	pub preview: EggPreview,
	pub genome: Option<usize>,

	pub preview_backup: EggPreview,

	pub sprites: Vec<usize>,
	pub bodydata: Vec<usize>,
	pub genetics: Vec<usize>,

	pub use_all_files: bool
}

#[derive(Clone)]
pub enum EggPreview {
	None,
	Manual {
		sprite_male: usize,
		sprite_female: usize,
		animation: String
	}
}

impl EggTag {
	pub fn new() -> EggTag {
		EggTag {
			name: "Untitled Egg".to_string(),
			version: "".to_string(),

			preview: EggPreview::None,
			genome: None,

			preview_backup: EggPreview::None,

			sprites: Vec::new(),
			bodydata: Vec::new(),
			genetics: Vec::new(),

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
