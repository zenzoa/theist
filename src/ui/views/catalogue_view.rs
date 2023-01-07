use crate::ui::icons::*;
use crate::ui::messages::Message;
use crate::ui::messages::catalogue_message::CatalogueMessage;
use crate::agent::catalogue::*;

use iced::widget::{ row, column, Column, text, text_input, button, scrollable, horizontal_rule };
use iced::{ Alignment, Length, theme };

pub fn properties(catalogue: &Catalogue) -> Column<Message> {
	match catalogue {
		Catalogue::File{ filename } => {
			column![
				row![
					catalogue_icon(),
					text(format!("Catalogue \"{}\"", &filename.string)).width(Length::Fill),
					button(delete_icon())
						.on_press(Message::Catalogue(CatalogueMessage::Remove))
						.style(theme::Button::Destructive)
				].spacing(5).align_items(Alignment::Center),
				horizontal_rule(1)
			].padding(30).spacing(20)
		},
		Catalogue::Inline{ filename, entries } => {
			let mut entry_list = column![
				row![
					text(format!("Entries ({})", entries.len())).width(Length::Fill),
					button(add_icon()).on_press(Message::Catalogue(CatalogueMessage::AddEntry))
				].spacing(5).align_items(Alignment::Center)
			].spacing(20);

			for (i, entry) in entries.iter().enumerate() {
				let set_classifier = move |new_classifier: String| -> Message {
					Message::Catalogue(CatalogueMessage::SetEntryClassifier(i, new_classifier))
				};
				let set_name = move |new_name: String| -> Message {
					Message::Catalogue(CatalogueMessage::SetEntryName(i, new_name))
				};
				let set_description = move |new_description: String| -> Message {
					Message::Catalogue(CatalogueMessage::SetEntryDescription(i, new_description))
				};
				let first_row = if entries.len() > 1 {
					row![
						text_input("Name", &entry.name, set_name).width(Length::Fill),
						button(up_icon()).on_press(Message::Catalogue(CatalogueMessage::MoveEntryUp(i))),
						button(down_icon()).on_press(Message::Catalogue(CatalogueMessage::MoveEntryDown(i))),
						button(delete_icon()).on_press(Message::Catalogue(CatalogueMessage::RemoveEntry(i)))
					]
				} else {
					row![
						text_input("Name", &entry.name, set_name).width(Length::Fill)
					]
				};
				entry_list = entry_list.push(
					column![
						first_row.spacing(5).align_items(Alignment::Center),
						row![
								text("Classifier").width(Length::FillPortion(1)),
								text_input("0 0 0000", &entry.classifier, set_classifier).width(Length::FillPortion(3))
							].spacing(5).align_items(Alignment::Center),
						row![
								text("Description").width(Length::FillPortion(1)),
								text_input("About this thing", &entry.description, set_description).width(Length::FillPortion(3))
							].spacing(5).align_items(Alignment::Center),
					].spacing(5)
				);
			}

			column![
				column![
					row![
						catalogue_icon(),
						text(format!("Catalogue \"{}\"", &filename.title)).width(Length::Fill),
						button(delete_icon())
							.on_press(Message::Catalogue(CatalogueMessage::Remove))
							.style(theme::Button::Destructive)
					].spacing(5).align_items(Alignment::Center),
					horizontal_rule(1),
				].padding([30, 30, 0, 30]).spacing(20),
				scrollable(
					column![
						text_input("Name", &filename.title, |x| Message::Catalogue(CatalogueMessage::SetName(x))),
						entry_list
					].padding(30).spacing(20)
				).height(Length::Fill)
			]
		}
	}
}

pub fn list(catalogues: &CatalogueList, selected_index: Option<usize>) -> Column<Message> {
	let mut catalogue_list = column![
		row![
			catalogue_icon(),
			text(format!("Catalogues ({})", catalogues.len()))
		].spacing(5),
	].spacing(10);

	for (i, catalogue) in catalogues.iter().enumerate() {
		let selected = if let Some(index) = selected_index { i == index } else { false };
		let filename = match catalogue {
			Catalogue::File{ filename } => filename,
			Catalogue::Inline{ filename, .. } => filename
		};
		let mut catalogue_row = row![
			button(filename.string.as_str())
				.on_press(Message::Catalogue(CatalogueMessage::Select(i)))
				.style(if selected { theme::Button::Primary } else { theme::Button::Secondary })
				.width(Length::Fill)
		].spacing(5).align_items(Alignment::Center);
		if catalogues.len() > 1 {
			catalogue_row = catalogue_row.push(
				button(up_icon())
					.on_press(Message::Catalogue(CatalogueMessage::MoveUp(i)))
					.style(theme::Button::Secondary));
			catalogue_row = catalogue_row.push(
				button(down_icon())
					.on_press(Message::Catalogue(CatalogueMessage::MoveDown(i)))
					.style(theme::Button::Secondary));
		}
		catalogue_list = catalogue_list.push(catalogue_row);
	}

	catalogue_list
}
