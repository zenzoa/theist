use crate::ui::{ Main, Selection };
use crate::ui::message::Message;
use crate::ui::message::tag::TagMessage;
use crate::ui::icon::*;
use crate::agent::tag::Tag;
use crate::agent::file::{ CreaturesFile, FileType };

use iced::widget::{ button, Column, column, horizontal_rule, Row, row, scrollable, Text, text };
use iced::{ Alignment, Length, theme };

pub fn view(main: &Main) -> Column<Message> {
	if let Some(tag) = main.get_selected_tag() {
		let mut file_lists = column![ text("Files:") ].spacing(20).padding(20);

		let add_button = button(row![add_icon(), text("Add New File")].spacing(5))
			.on_press(Message::Tag(TagMessage::AddFile))
			.width(Length::Fill)
			.style(theme::Button::Secondary);

		let add_existing_button = button(row![add_icon(), text("Add Existing File")].spacing(5))
			.on_press(Message::ShowExistingFileDialog)
			.width(Length::Fill)
			.style(theme::Button::Secondary);

		let mut extra_add_buttons = row![].spacing(5);

		match tag {
			Tag::Agent(agent_tag) => {
				if !agent_tag.scripts.is_empty() {
					let selected_script_index = if let Selection::Script(index) = main.selection { Some(index) } else { None };
					file_lists = file_lists.push(file_list_header(FileType::Script));
					file_lists = file_lists.push(file_list(&main.files, &agent_tag.scripts, selected_script_index));
				}

				if !agent_tag.sprites.is_empty() {
					let selected_sprite_index = if let Selection::Sprite(index) = main.selection { Some(index) } else { None };
					file_lists = file_lists.push(file_list_header(FileType::Sprite));
					file_lists = file_lists.push(file_list(&main.files, &agent_tag.sprites, selected_sprite_index));
				}

				if !agent_tag.sounds.is_empty() {
					let selected_sound_index = if let Selection::Sound(index) = main.selection { Some(index) } else { None };
					file_lists = file_lists.push(file_list_header(FileType::Sound));
					file_lists = file_lists.push(file_list(&main.files, &agent_tag.sounds, selected_sound_index));
				}

				if !agent_tag.catalogues.is_empty() {
					let selected_catalogue_index = if let Selection::Catalogue(index) = main.selection { Some(index) } else { None };
					file_lists = file_lists.push(file_list_header(FileType::Catalogue));
					file_lists = file_lists.push(file_list(&main.files, &agent_tag.catalogues, selected_catalogue_index));
				}

				if agent_tag.scripts.is_empty() && agent_tag.sprites.is_empty() && agent_tag.sounds.is_empty() && agent_tag.catalogues.is_empty() {
					file_lists = file_lists.push(text("(no files yet)"));
				}

				extra_add_buttons = extra_add_buttons.push(
					button(row![add_icon(), text("Add Inline Catalogue")].spacing(5))
						.on_press(Message::Tag(TagMessage::AddInlineCatalogue))
						.width(Length::Fill)
						.style(theme::Button::Secondary)
				);
			},

			Tag::Egg(egg_tag) => {
				if !egg_tag.genetics.is_empty() {
					let selected_genetics_index = if let Selection::Genetics(index) = main.selection { Some(index) } else { None };
					file_lists = file_lists.push(file_list_header(FileType::Genetics));
					file_lists = file_lists.push(file_list(&main.files, &egg_tag.genetics, selected_genetics_index));
				}

				if !egg_tag.sprites.is_empty() {
					let selected_sprite_index = if let Selection::Sprite(index) = main.selection { Some(index) } else { None };
					file_lists = file_lists.push(file_list_header(FileType::Sprite));
					file_lists = file_lists.push(file_list(&main.files, &egg_tag.sprites, selected_sprite_index));
				}

				if !egg_tag.bodydata.is_empty() {
					let selected_bodydata_index = if let Selection::BodyData(index) = main.selection { Some(index) } else { None };
					file_lists = file_lists.push(file_list_header(FileType::BodyData));
					file_lists = file_lists.push(file_list(&main.files, &egg_tag.bodydata, selected_bodydata_index));
				}

				if egg_tag.genetics.is_empty() && egg_tag.sprites.is_empty() && egg_tag.bodydata.is_empty() {
					file_lists = file_lists.push(text("(no files yet)"));
				}
			},

			Tag::Free(_free_tag) => {
				return column![ text("(this type of tag doesn't have files)") ].padding(20);
			}
		}

		return column![
			scrollable(file_lists).height(Length::Fill),
			horizontal_rule(1),
			column![ add_button, add_existing_button, extra_add_buttons ].spacing(5).padding(20)
		];
	} else {
		column![ text("(no tag selected)") ].padding(20)
	}
}

pub fn existing_file_list(main: &Main) -> Column<Message> {
	let mut existing_files = column![].spacing(10);
	for (k, v) in main.files.iter().enumerate() {
		let icon = file_icon(v.get_filetype());
		let button_content = row![ icon, text(v.get_output_filename()) ].spacing(5);
		let file_button = button(button_content)
			.on_press(Message::Tag(TagMessage::AddExistingFile(k)))
			.width(Length::Fill);
		existing_files = existing_files.push(file_button);
	}
	existing_files
}

fn section_header<'a>(icon: Text<'a>, title: &'a str) -> Row<'a, Message> {
	row![ icon, text(title) ].spacing(5).align_items(Alignment::Center)
}

fn file_icon(filetype: FileType) -> Text<'static> {
	match filetype {
		FileType::Script => script_icon(),
		FileType::Sprite => sprite_icon(),
		FileType::Sound => sound_icon(),
		FileType::Catalogue => catalogue_icon(),
		FileType::BodyData => bodydata_icon(),
		FileType::Genetics => genetics_icon()
	}
}

fn file_list_header<'a>(filetype: FileType) -> Row<'a, Message> {
	let icon = file_icon(filetype);
	match filetype {
		FileType::Script => section_header(icon, "Scripts"),
		FileType::Sprite => section_header(icon, "Sprites"),
		FileType::Sound => section_header(icon, "Sounds"),
		FileType::Catalogue => section_header(icon, "Catalogues"),
		FileType::BodyData => section_header(icon, "Body Data"),
		FileType::Genetics => section_header(icon, "Genomes")
	}
}

fn file_list<'a>(
	files: &'a [CreaturesFile],
	file_indexes: &'a Vec<usize>,
	selected_index: Option<usize>
) -> Column<'a, Message>
{
	let mut list = column![].spacing(10);
	for (i, file_index) in file_indexes.iter().enumerate() {
		if let Some(file) = files.get(*file_index) {
			let selected = if let Some(index) = selected_index { index == i } else { false };
			list = list.push(file_button(file, i, file_indexes.len() > 1, selected));
		}
	}
	list
}

fn file_button(
	file: &CreaturesFile,
	index: usize,
	multiple_files: bool,
	selected: bool
) -> Row<Message>
{
	let missing_data = matches!(file.get_data(), None);

	let mut file_button_row = row![
		button(file.get_output_filename_ref().as_str())
			.on_press(Message::Tag(TagMessage::SelectFile(file.get_filetype(), index)))
			.width(Length::Fill)
			.style(
				if selected {
					theme::Button::Primary
				} else if missing_data {
					theme::Button::Destructive
				} else {
					theme::Button::Secondary
				})
	].spacing(5).width(Length::Fill);

	if multiple_files {
		file_button_row = file_button_row.push(
			button(up_icon())
				.on_press(Message::Tag(TagMessage::MoveFileUp(file.get_filetype(), index)))
				.style(theme::Button::Secondary));
		file_button_row = file_button_row.push(
			button(down_icon())
				.on_press(Message::Tag(TagMessage::MoveFileDown(file.get_filetype(), index)))
				.style(theme::Button::Secondary));
	}

	file_button_row = file_button_row.push(
		button(delete_icon())
			.on_press(Message::Tag(TagMessage::RemoveFile(file.get_filetype(), index, file.get_output_filename())))
			.style(theme::Button::Secondary));

	file_button_row
}
