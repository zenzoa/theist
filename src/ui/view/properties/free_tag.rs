use crate::ui::Main;
use crate::ui::message::Message;
use crate::ui::icon::*;
use crate::agent::free_tag::FreeTag;
use crate::file_helper;

use iced::widget::{ button, Column, column, horizontal_rule, horizontal_space, Row, row, text, text_input };
use iced::{ Alignment, Command, Element, Event, executor, Length, subscription, Subscription, Theme, theme };

pub fn free_tag_props<'a>(main: &'a Main, free_tag: &'a FreeTag) -> Column<'a, Message> {
	// just give big ol textbox with contents in
	column![ text("free tag props") ]
}
