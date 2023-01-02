use crate::agent::*;

use bytes::Bytes;

#[derive(Clone)]
pub struct EggTag {
	pub filepath: String,
	pub name: String,
	pub version: String,

	pub preview_sprite_female: String,
	pub preview_sprite_male: String,
	pub preview_animation: String,

	pub genetics: GeneticsList,
	pub sprites: SpriteList,
	pub body_data: BodyDataList,

	pub genetics_files: Vec<Bytes>,
	pub sprite_files: Vec<Bytes>,
	pub body_data_files: Vec<Bytes>,
}

impl EggTag {
	pub fn new(name: String) -> EggTag {
		EggTag {
			filepath: String::from(""),
			name,
			version: String::from(""),
			preview_sprite_female: String::from(""),
			preview_sprite_male: String::from(""),
			preview_animation: String::from(""),
			genetics: GeneticsList::new(),
			sprites: SpriteList::new(),
			body_data: BodyDataList::new(),
			genetics_files: Vec::new(),
			sprite_files: Vec::new(),
			body_data_files: Vec::new()
		}
	}
}
