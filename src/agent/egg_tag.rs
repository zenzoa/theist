use crate::agent::*;

use bytes::Bytes;

#[derive(Clone)]
pub struct EggTag {
	pub filepath: String,
	pub name: String,
	pub version: String,

	pub preview_sprite_male: String,
	pub preview_sprite_female: String,
	pub preview_animation: String,

	// pub genetics: Vec<Genetics>,
	pub sprites: Vec<Sprite>,

	// pub genetic_files: Vec<Bytes>,
	pub sprite_files: Vec<Bytes>
}

impl EggTag {
	pub fn new() -> EggTag {
		EggTag {
			filepath: String::from(""),
			name: String::from(""),
			version: String::from(""),
			preview_sprite_male: String::from(""),
			preview_sprite_female: String::from(""),
			preview_animation: String::from(""),
			sprites: Vec::new(),
			sprite_files: Vec::new()
		}
	}
}
