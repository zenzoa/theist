use crate::ui::Main;
use crate::ui::message::Message;
use crate::ui::message::tag::TagMessage;
use crate::ui::icon::*;

use iced::widget::{ button, horizontal_space, Row, row, text };
use iced::{ Alignment, Length, theme };

pub fn view(main: &Main) -> Row<Message> {
	let mut tag_list = row![
		text("Tags:"),
		horizontal_space(Length::Fixed(1.0))
	].padding(20).spacing(5).align_items(Alignment::Center);

	for (i, tag) in main.tags.iter().enumerate() {
		let mut selected = false;
		if let Some(index) = main.selected_tag_index {
			if index == i {
				selected = true;
			}
		}
		tag_list = tag_list.push(
			button(tag.get_name().as_str())
				.on_press(Message::Tag(TagMessage::Select(i)))
				.style(if selected { theme::Button::Primary } else { theme::Button::Secondary })
		);
	}

	tag_list = tag_list.push(
		button(add_icon())
			.on_press(Message::ShowNewTagDialog)
			.style(theme::Button::Secondary)
	);

	tag_list
}
