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
                set_label: self.note.display_filename().as_str(),
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
