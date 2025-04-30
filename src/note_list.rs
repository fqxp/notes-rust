use gtk::prelude::*;
use relm4::prelude::*;

pub struct NoteListItem {
    filename: String,
}

#[derive(Debug)]
pub enum NoteListMsg {
    RevealFile(String),
}

#[derive(Debug)]
pub enum NoteListOutput {
    SelectFile(String),
}

#[relm4::factory(pub)]
impl FactoryComponent for NoteListItem {
    type Init = String;
    type Input = NoteListMsg;
    type Output = NoteListOutput;
    type CommandOutput = ();
    type ParentWidget = gtk::Box;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 10,

            #[name(open_button)]
            gtk::Button {
                #[watch]
                set_label: &self.filename,
                connect_clicked[sender,filename=self.filename.clone()] => move |_| {
                    let _ = sender.output(NoteListOutput::SelectFile(filename.clone()));
                }
            }
        }
    }

    fn init_model(
        init: Self::Init,
        _index: &Self::Index,
        _sender: relm4::FactorySender<Self>,
    ) -> Self {
        Self { filename: init }
    }
}
