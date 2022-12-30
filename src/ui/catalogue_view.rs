use crate::ui::*;
use crate::agent::catalogue::*;

use iced::widget::{ row, column, Column, text, text_input, button, scrollable, horizontal_rule };
use iced::{ Alignment, Length };

pub fn properties(catalogue: &Catalogue) -> Column<Message> {
	match catalogue {
		Catalogue::File{ filename } => {
			column![
				text(format!("Catalogue \"{}\"", &filename.title)),
				horizontal_rule(1)
			].padding(20).spacing(20)
		},
		Catalogue::Inline{ filename, entries } => {
			let mut entry_list = column![
				row![
					text(format!("Entries ({})", entries.len())).width(Length::Fill),
					button("+").on_press(Message::AddCatalogueEntry)
				].spacing(5).align_items(Alignment::Center)
			].spacing(20);

			for (i, entry) in entries.iter().enumerate() {
				let set_classifier = move |new_classifier: String| -> Message {
					Message::SetCatalogueEntryClassifier(i, new_classifier)
				};
				let set_name = move |new_name: String| -> Message {
					Message::SetCatalogueEntryName(i, new_name)
				};
				let set_description = move |new_description: String| -> Message {
					Message::SetCatalogueEntryDescription(i, new_description)
				};
				let first_row = if entries.len() > 1 {
					row![
						text_input("Name", &entry.name, set_name).width(Length::Fill),
						button("^").on_press(Message::MoveCatalogueEntryUp(i)),
						button("v").on_press(Message::MoveCatalogueEntryDown(i)),
						button("x").on_press(Message::DeleteCatalogueEntry(i))
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
					text(format!("Catalogue \"{}\"", &filename.title)),
					horizontal_rule(1),
				].padding([20, 20, 0, 20]).spacing(20),
				scrollable(
					column![
						text_input("Name", &filename.title, Message::SetCatalogueName),
						entry_list
					].padding(20).spacing(20)
				).height(Length::Fill)
			]
		}
	}
}
