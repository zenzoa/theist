use crate::ui::Main;
use crate::ui::message::Message;
use crate::ui::message::catalogue::CatalogueMessage;
use crate::ui::icon::*;
use crate::agent::catalogue::Catalogue;
use crate::file_helper;

use iced::widget::{ button, Column, column, row, text, text_input };
use iced::{ Alignment, Length, theme };

pub fn catalogue_props<'a>(_main: &'a Main, catalogue: &'a Catalogue) -> Column<'a, Message> {
	match catalogue {
		Catalogue::Raw{ input_filename, output_filename, .. } =>
			column![
				text("Catalogue Properties:"),
				text(format!("Input file: {}", input_filename).as_str()),
				row![
					text("Output file:"),
					text_input("Output File Name", &file_helper::title(output_filename),
						|x| Message::Catalogue(CatalogueMessage::SetName(x))),
					text(".catalogue")
				].spacing(5).align_items(Alignment::Center)
			],

		Catalogue::Inline{ output_filename, entries, .. } => {
			let mut props = column![
				text("Catalogue Properties:"),
				text("Inline catalogue"),
				row![
					text("Output file:"),
					text_input("Output File Name", &file_helper::title(output_filename),
						|x| Message::Catalogue(CatalogueMessage::SetName(x))),
					text(".catalogue")
				].spacing(5).align_items(Alignment::Center)
			];

			let mut entry_list = column![
				row![
					text("Entries:").width(Length::Fill),
					button(add_icon())
						.on_press(Message::Catalogue(CatalogueMessage::AddEntry))
						.style(theme::Button::Secondary)
				].align_items(Alignment::Center)
			].spacing(10);

			for (i, entry) in entries.iter().enumerate() {
				let mut entry_column = column![].spacing(5);
				let mut entry_row = row![
					text_input("Name", entry.name.as_str(),
						move |x| Message::Catalogue(CatalogueMessage::SetEntryName(i, x)))
						.width(Length::FillPortion(1)),
					text_input("Classifier", entry.classifier.as_str(),
						move |x| Message::Catalogue(CatalogueMessage::SetEntryClassifier(i, x)))
						.width(Length::FillPortion(1))
				].spacing(5).align_items(Alignment::Center);

				if entries.len() > 1 {
					entry_row = entry_row.push(
						button(up_icon())
							.on_press(Message::Catalogue(CatalogueMessage::MoveEntryUp(i)))
							.style(theme::Button::Secondary));
					entry_row = entry_row.push(
						button(down_icon())
							.on_press(Message::Catalogue(CatalogueMessage::MoveEntryDown(i)))
							.style(theme::Button::Secondary));
					entry_row = entry_row.push(
						button(delete_icon())
							.on_press(Message::Catalogue(CatalogueMessage::RemoveEntry(i)))
							.style(theme::Button::Secondary));
				}

				entry_column = entry_column.push(entry_row);
				entry_column = entry_column.push(
					text_input("Description", entry.description.as_str(),
						move |x| Message::Catalogue(CatalogueMessage::SetEntryDescription(i, x)))
						.width(Length::Fill)).padding([0, 0, 10, 0]);
				entry_list = entry_list.push(entry_column);
			}

			props = props.push(entry_list);
			props
		}
	}
}
