use crate::ui::Main;
use crate::ui::alert::Alert;
use crate::ui::message::Message;
use crate::ui::icon::*;

use iced::widget::{ button, Column, column, row, text };
use iced::{ Length, theme };

pub fn view(main: &Main) -> Column<Message> {
	if main.alerts.is_empty() {

		column![]

	} else {

		let mut alerts = column![].spacing(5).padding(10);

		for (i, alert) in main.alerts.iter().enumerate() {
			match alert {
				Alert::Update(message) => {
					alerts = alerts.push(
						button(
							row![
								alert_icon(),
								text(&message).width(Length::Fill),
								dismiss_icon()
							].spacing(5))
							.on_press(Message::DismissAlert(i))
							.style(theme::Button::Positive)
					)
				},
				Alert::Error(message) => {
					alerts = alerts.push(
						button(
							row![
								error_icon(),
								text(&message).width(Length::Fill),
								dismiss_icon()
							].spacing(5))
							.on_press(Message::DismissAlert(i))
							.style(theme::Button::Destructive)
					)
				}
			}
		}

		alerts

	}
}
