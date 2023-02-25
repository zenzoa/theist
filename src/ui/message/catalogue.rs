use crate::ui::Main;
use crate::ui::dialog::*;
use crate::agent::file::CreaturesFile;
use crate::agent::catalogue::{ Catalogue, CatalogueEntry };

#[derive(Debug, Clone)]
pub enum CatalogueMessage {
	SetName(String),
	AddEntry,
	RemoveEntry(usize),
	MoveEntryUp(usize),
	MoveEntryDown(usize),
	SetEntryName(usize, String),
	SetEntryClassifier(usize, String),
	SetEntryDescription(usize, String)
}

pub fn check_catalogue_message(main: &mut Main, message: CatalogueMessage) {
	if let Some(CreaturesFile::Catalogue(catalogue)) = main.get_selected_file_mut() {
		match message {
			CatalogueMessage::SetName(new_name) => {
				match catalogue {
					Catalogue::Raw{ output_filename, .. } => {
						*output_filename = format!("{}.catalogue", new_name);
					},
					Catalogue::Inline{ output_filename, .. } => {
						*output_filename = format!("{}.catalogue", new_name);
					}
				}
				main.modified = true;
			},

			CatalogueMessage::AddEntry => {
				let new_entry = CatalogueEntry{
					name: "".to_string(),
					classifier: "".to_string(),
					description: "".to_string()
				};
				catalogue.add_entry(new_entry);
				main.modified = true;
			},

			CatalogueMessage::RemoveEntry(entry_index) => {
				if confirm_remove_entry() && catalogue.remove_entry(entry_index) {
					main.modified = true;
				}
			},

			CatalogueMessage::MoveEntryUp(entry_index) => {
				if catalogue.move_entry_up(entry_index) {
					main.modified = true;
				}
			},

			CatalogueMessage::MoveEntryDown(entry_index) => {
				if catalogue.move_entry_down(entry_index) {
					main.modified = true;
				}
			},

			CatalogueMessage::SetEntryName(entry_index, new_name) => {
				if let Catalogue::Inline{ entries, .. } = catalogue {
					if let Some(entry) = entries.get_mut(entry_index) {
						entry.name = new_name;
						main.modified = true;
					}
				}
			},

			CatalogueMessage::SetEntryClassifier(entry_index, new_classifier) => {
				if let Catalogue::Inline{ entries, .. } = catalogue {
					if let Some(entry) = entries.get_mut(entry_index) {
						entry.classifier = new_classifier;
						main.modified = true;
					}
				}
			},

			CatalogueMessage::SetEntryDescription(entry_index, new_description) => {
				if let Catalogue::Inline{ entries, .. } = catalogue {
					if let Some(entry) = entries.get_mut(entry_index) {
						entry.description = new_description;
						main.modified = true;
					}
				}
			}
		}
	}
}

