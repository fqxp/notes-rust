use gtk::prelude::*;
use relm4::prelude::*;

use crate::storage::Note;

pub struct NoteListItem {
    pub note: Note,
}

#[derive(Debug)]
pub enum NoteListMsg {
    RevealFile(String),
}

#[derive(Debug)]
pub enum NoteListOutput {
    SelectFile(usize),
}

fn display_filename(filename: &String) -> &str {
    let base_filename = filename.rsplit("/").next().unwrap();

    return match base_filename.strip_suffix(".md") {
        Some(name) => name,
        None => base_filename,
    };
}

#[relm4::factory(pub)]
impl FactoryComponent for NoteListItem {
    type Init = Note;
    type Input = NoteListMsg;
    type Output = NoteListOutput;
    type CommandOutput = ();
    type ParentWidget = gtk::ListBox;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 10,
            set_margin_top: 10,
            set_margin_bottom: 10,

            #[name(open_button)]
            gtk::Label {
                set_margin_top: 4,
                set_margin_bottom: 4,
                #[watch]
                set_label: display_filename(&self.note.name),
            },
        }
    }

    fn init_model(
        init: Self::Init,
        _index: &Self::Index,
        _sender: relm4::FactorySender<Self>,
    ) -> Self {
        Self { note: init }
    }
}
