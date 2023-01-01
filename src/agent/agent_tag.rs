use crate::agent::*;
use crate::agent::background::*;

use std::fmt;
use bytes::Bytes;

#[derive(Clone)]
pub struct AgentTag {
	pub filepath: String,
	pub name: String,
	pub version: String,
	pub description: String,
	pub supported_game: SupportedGame,
	pub removescript: RemoveScript,
	pub preview: Preview,

	pub scripts: ScriptList,
	pub sprites: SpriteList,
	pub backgrounds: BackgroundList,
	pub sounds: SoundList,
	pub catalogues: CatalogueList,

	pub script_files: Vec<Bytes>,
	pub sprite_files: Vec<Bytes>,
	pub background_files: Vec<Bytes>,
	pub sound_files: Vec<Bytes>,
	pub catalogue_files: Vec<Bytes>
}

impl AgentTag {
	pub fn new(name: String) -> AgentTag {
		AgentTag {
			filepath: String::from(""),
			name,
			version: String::from(""),
			description: String::from(""),
			supported_game: SupportedGame::C3DS,
			removescript: RemoveScript::Auto,
			preview: Preview::Auto,

			scripts: ScriptList::new(),
			sprites: SpriteList::new(),
			backgrounds: BackgroundList::new(),
			sounds: SoundList::new(),
			catalogues: CatalogueList::new(),

			script_files: Vec::new(),
			sprite_files: Vec::new(),
			background_files: Vec::new(),
			sound_files: Vec::new(),
			catalogue_files: Vec::new()
		}
	}

	pub fn add_inline_catalogue(&mut self) {
		self.catalogues.push(
			Catalogue::Inline{
				filename: Filename::new("my_catalogue.catalogue"),
				entries: vec![
					CatalogueEntry::new("0 0 0000", self.name.as_str(), self.description.as_str())
				]
			}
		);
	}
}

#[derive(Clone, PartialEq)]
pub enum RemoveScript {
	None,
	Auto,
	Manual(String)
}

impl fmt::Display for RemoveScript {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match &self {
			RemoveScript::None => write!(f, ""),
			RemoveScript::Auto => write!(f, "auto"),
			RemoveScript::Manual(s) => write!(f, "{}", s),
		}
	}
}

#[derive(Clone, PartialEq)]
pub enum Preview {
	Auto,
	Manual { sprite: String, animation: String }
}
