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

	pub fn convert_to_agent(&self) -> AgentTag {
		let mut agent_tag = AgentTag::new(self.name.clone());
		agent_tag.filepath = self.filepath.clone();
		agent_tag.version = self.version.clone();
		agent_tag.sprites = self.sprites.clone();
		let first_sprite_title = if let Some(sprite) = &self.sprites.get(0) {
				sprite.get_title()
			} else {
				String::from("")
			};
		let female_is_default = self.preview_sprite_female.is_empty() || self.preview_sprite_female == first_sprite_title;
		let male_is_default = self.preview_sprite_male.is_empty() || self.preview_sprite_male == first_sprite_title;
		let animation_is_default = self.preview_animation.is_empty() || self.preview_animation == "0";
		if female_is_default && male_is_default && animation_is_default {
			agent_tag.preview = Preview::Auto;
		} else {
			let preview_sprite = if !female_is_default {
					self.preview_sprite_female.clone()
				} else if !male_is_default {
					self.preview_sprite_male.clone()
				} else {
					first_sprite_title
				};
			agent_tag.preview = Preview::Manual{
				sprite: preview_sprite,
				animation: self.preview_animation.clone()
			};
		}
		agent_tag
	}
}
