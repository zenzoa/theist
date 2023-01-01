use crate::agent::tag::*;
use crate::agent::catalogue::*;
use crate::ui::{ Main, SelectionType };
use crate::ui::dialogs::*;

#[derive(Debug, Clone)]
pub enum CatalogueMessage {
	AddInlineCatalogue,
	Select(usize),
	Remove(usize),
	MoveUp(usize),
	MoveDown(usize),
	SetName(String),
	AddEntry,
	RemoveEntry(usize),
	MoveEntryUp(usize),
	MoveEntryDown(usize),
	SetEntryClassifier(usize, String),
	SetEntryName(usize, String),
	SetEntryDescription(usize, String),
}

pub fn check_catalogue_message(main: &mut Main, message: CatalogueMessage) {
	match message {
		CatalogueMessage::AddInlineCatalogue => {
			if let Some(selected_tag) = main.selected_tag {
				if let Tag::Agent(tag) = &mut main.tags[selected_tag] {
					tag.add_inline_catalogue();
					main.selection_type = SelectionType::Catalogue(tag.catalogues.len() - 1);
					main.modified = true;
				}
			}
		},

		CatalogueMessage::Select(index) => {
			main.selection_type = SelectionType::Catalogue(index);
		},

		CatalogueMessage::Remove(index) => {
			if confirm_remove_item("catalogue") {
				if let Some(selected_tag) = main.selected_tag {
					if let Tag::Agent(tag) = &mut main.tags[selected_tag] {
						tag.catalogues.remove(index);
						main.selection_type = SelectionType::Tag;
						main.modified = true;
					}
				}
			}
		},

		CatalogueMessage::MoveUp(index) => {
			if let Some(selected_tag) = main.selected_tag {
				if let Tag::Agent(tag) = &mut main.tags[selected_tag] {
					tag.catalogues.move_up(index);
					main.modified = true;
				}
			}
		},

		CatalogueMessage::MoveDown(index) => {
			if let Some(selected_tag) = main.selected_tag {
				if let Tag::Agent(tag) = &mut main.tags[selected_tag] {
					tag.catalogues.move_down(index);
					main.modified = true;
				}
			}
		},

		CatalogueMessage::SetName(new_name) => {
			if let Some(selected_tag) = main.selected_tag {
				if let SelectionType::Catalogue(catalogue_index) = main.selection_type {
					if let Tag::Agent(tag) = &mut main.tags[selected_tag] {
						if let Some(catalogue) = &mut tag.catalogues.get_mut(catalogue_index) {
							catalogue.set_name(new_name);
							main.modified = true;
						}
					}
				}
			}
		},

		CatalogueMessage::AddEntry => {
			if let Some(selected_tag) = main.selected_tag {
				if let SelectionType::Catalogue(catalogue_index) = main.selection_type {
					if let Tag::Agent(tag) = &mut main.tags[selected_tag] {
						if let Some(catalogue) = &mut tag.catalogues.get_mut(catalogue_index) {
							catalogue.add_entry(CatalogueEntry::new("0 0 0000", "New Entry", "Something descriptive"));
							main.modified = true;
						}
					}
				}
			}
		},

		CatalogueMessage::RemoveEntry(entry_index) => {
			if let Some(selected_tag) = main.selected_tag {
				if let SelectionType::Catalogue(catalogue_index) = main.selection_type {
					if let Tag::Agent(tag) = &mut main.tags[selected_tag] {
						if let Some(catalogue) = &mut tag.catalogues.get_mut(catalogue_index) {
							catalogue.remove_entry(entry_index);
							main.modified = true;
						}
					}
				}
			}
		},

		CatalogueMessage::MoveEntryUp(entry_index) => {
			if let Some(selected_tag) = main.selected_tag {
				if let SelectionType::Catalogue(catalogue_index) = main.selection_type {
					if let Tag::Agent(tag) = &mut main.tags[selected_tag] {
						if let Some(catalogue) = &mut tag.catalogues.get_mut(catalogue_index) {
							catalogue.move_entry_up(entry_index);
							main.modified = true;
						}
					}
				}
			}
		},

		CatalogueMessage::MoveEntryDown(entry_index) => {
			if let Some(selected_tag) = main.selected_tag {
				if let SelectionType::Catalogue(catalogue_index) = main.selection_type {
					if let Tag::Agent(tag) = &mut main.tags[selected_tag] {
						if let Some(catalogue) = &mut tag.catalogues.get_mut(catalogue_index) {
							catalogue.move_entry_down(entry_index);
							main.modified = true;
						}
					}
				}
			}
		},

		CatalogueMessage::SetEntryClassifier(entry_index, new_classifier) => {
			if let Some(selected_tag) = main.selected_tag {
				if let SelectionType::Catalogue(catalogue_index) = main.selection_type {
					if let Tag::Agent(tag) = &mut main.tags[selected_tag] {
						if let Some(catalogue) = &mut tag.catalogues.get_mut(catalogue_index) {
							catalogue.set_entry_classifier(entry_index, new_classifier);
							main.modified = true;
						}
					}
				}
			}
		},

		CatalogueMessage::SetEntryName(entry_index, new_name) => {
			if let Some(selected_tag) = main.selected_tag {
				if let SelectionType::Catalogue(catalogue_index) = main.selection_type {
					if let Tag::Agent(tag) = &mut main.tags[selected_tag] {
						if let Some(catalogue) = &mut tag.catalogues.get_mut(catalogue_index) {
							catalogue.set_entry_name(entry_index, new_name);
							main.modified = true;
						}
					}
				}
			}
		},

		CatalogueMessage::SetEntryDescription(entry_index, new_description) => {
			if let Some(selected_tag) = main.selected_tag {
				if let SelectionType::Catalogue(catalogue_index) = main.selection_type {
					if let Tag::Agent(tag) = &mut main.tags[selected_tag] {
						if let Some(catalogue) = &mut tag.catalogues.get_mut(catalogue_index) {
							catalogue.set_entry_description(entry_index, new_description);
							main.modified = true;
						}
					}
				}
			}
		},
	}
}
