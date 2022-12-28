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
				entry_list = entry_list.push(
					column![
						row![
								text_input("Name", &entry.name, Message::SetCatalogueEntryName).width(Length::Fill),
								button("^").on_press(Message::MoveCatalogueEntryUp(i)),
								button("v").on_press(Message::MoveCatalogueEntryDown(i)),
								button("x").on_press(Message::DeleteCatalogueEntry(i))
							].spacing(5).align_items(Alignment::Center),
						row![
								text("Classifier").width(Length::FillPortion(1)),
								text_input("0 0 0000", &entry.classifier, Message::SetCatalogueEntryClassifier).width(Length::FillPortion(3))
							].spacing(5).align_items(Alignment::Center),
						row![
								text("Description").width(Length::FillPortion(1)),
								text_input("About this thing", &entry.description, Message::SetCatalogueEntryDescription).width(Length::FillPortion(3))
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
