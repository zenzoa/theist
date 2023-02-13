use crate::ui::Main;
use crate::ui::message::Message;
use crate::agent::file::CreaturesFile;
use crate::agent::tag::Tag;

use iced::widget::{ Column, column, text };

mod agent_tag;
mod egg_tag;
mod free_tag;
mod script;
mod sprite;
mod sound;
mod catalogue;
mod bodydata;
mod genetics;

pub fn view(main: &Main) -> Column<Message> {
	let mut properties = None;
	match main.get_selected_file() {
		Some(file) =>
			match file {
				CreaturesFile::Script(script) => {
					properties = Some(script::script_props(main, script))
				},
				CreaturesFile::Sprite(sprite) => {
					properties = Some(sprite::sprite_props(main, sprite))
				},
				CreaturesFile::Sound(sound) => {
					properties = Some(sound::sound_props(main, sound))
				},
				CreaturesFile::Catalogue(catalogue) => {
					properties = Some(catalogue::catalogue_props(main, catalogue))
				},
				CreaturesFile::BodyData(bodydata) => {
					properties = Some(bodydata::bodydata_props(main, bodydata))
				},
				CreaturesFile::Genetics(genetics) => {
					properties = Some(genetics::genetics_props(main, genetics))
				}
			},
		None =>
			if let Some(tag) = main.get_selected_tag() {
				match tag {
					Tag::Agent(agent_tag) => {
						properties = Some(agent_tag::agent_tag_props(main, agent_tag))
					},
					Tag::Egg(egg_tag) => {
						properties = Some(egg_tag::egg_tag_props(main, egg_tag))
					},
					Tag::Free(free_tag) => {
						properties = Some(free_tag::free_tag_props(main, free_tag))
					}
				}
			}
	}

	match properties {
		Some(props) => props.spacing(20),
		None => column![ text("( nothing selected )") ]
	}
}

