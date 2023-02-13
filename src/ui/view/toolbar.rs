use crate::ui::Main;
use crate::ui::message::Message;
use crate::ui::message::file::FileMessage;
use crate::ui::icon::*;

use iced::widget::{ button, horizontal_space, Row, row, text };
use iced::{ Length, theme };

pub fn view(_main: &Main) -> Row<Message> {
	row![

		button(row![new_icon(), text("New")].spacing(5))
			.on_press(Message::File(FileMessage::New))
			.style(theme::Button::Secondary),

		button(row![open_icon(), text("Open")]
			.spacing(5))
			.on_press(Message::File(FileMessage::Open))
			.style(theme::Button::Secondary),

		button(row![save_icon(), text("Save")].spacing(5))
			.on_press(Message::File(FileMessage::Save))
			.style(theme::Button::Secondary),

		button("Save As")
			.on_press(Message::File(FileMessage::SaveAs))
			.style(theme::Button::Secondary),

		horizontal_space(Length::Fill),

		button(row![compile_icon(), text("Compile")].spacing(5))
			.on_press(Message::File(FileMessage::Compile))
			.style(theme::Button::Secondary)

	].padding(10).spacing(5)
}
