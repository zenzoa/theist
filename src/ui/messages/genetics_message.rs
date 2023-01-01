// use crate::agent::tag::*;
use crate::ui::{ Main, SelectionType };

#[derive(Debug, Clone)]
pub enum GeneticsMessage {
	Select(usize),
	Remove(usize),
	MoveUp(usize),
	MoveDown(usize),
}

pub fn check_genetics_message(main: &mut Main, message: GeneticsMessage) {
	match message {
		_ => ()
	}
}
