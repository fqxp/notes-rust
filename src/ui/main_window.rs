use crate::ui::note_list::{NoteListItem, NoteListOutput};
use crate::{storage::FilesystemStorage, util::markdown::markdown_to_html};
use gtk::prelude::*;
use relm4::RelmListBoxExt;
use relm4::prelude::*;
use relm4::{ComponentParts, ComponentSender, SimpleComponent};
use std::path::Path;
use webkit6::prelude::WebViewExt;

pub struct AppModel {
    note_list: FactoryVecDeque<NoteListItem>,
    note_content: Option<String>,
    error: Option<String>,
}

#[derive(Debug)]
pub enum AppMsg {
    SelectFile(usize),
}

#[relm4::component(pub)]
impl SimpleComponent for AppModel {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    view! {
        #[root]
        gtk::Window {
            set_title: Some("simple app"),
            set_default_width: 100,
            set_default_height: 100,

            gtk::Box{
                set_orientation: gtk::Orientation::Vertical,
                gtk::Label {
                    #[watch]
                    set_label: model.error.as_deref().unwrap_or(""),
                },
                gtk::Paned::new(gtk::Orientation::Horizontal) {
                    set_position: 200,
                    set_wide_handle: true,

                    #[wrap(Some)]
                    set_start_child = &gtk::ScrolledWindow {
                        set_vexpand: true,

                        #[local_ref]
                        note_list_box -> gtk::ListBox {
                            connect_row_activated[sender] => move |list_box, row| {
                                let index = list_box.index_of_child(row).unwrap() as usize;
                                sender.input_sender().emit(AppMsg::SelectFile(index));
                            }
                        },
                    },

                    #[wrap(Some)]
                    set_end_child = match &model.note_content {
                        Some(markdown) => &webkit6::WebView {
                           set_vexpand: true,
                           #[watch]
                           load_html[None]: markdown_to_html(markdown).as_str()

                        }
                        None => {
                            &gtk::Label {
                                set_label: "no note loaded",
                            }
                        }
                    },
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let note_list = FactoryVecDeque::builder()
            .launch(gtk::ListBox::default())
            .forward(sender.input_sender(), |output| match output {
                NoteListOutput::SelectFile(index) => AppMsg::SelectFile(index),
            });

        let mut model = AppModel {
            note_list,
            note_content: None,
            error: None,
        };

        let storage = FilesystemStorage::new(Path::new("/home/frank/notes/persönlich"));
        let notes = storage.list().unwrap();
        for note in notes.into_iter() {
            model.note_list.guard().push_back(note);
        }

        let note_list_box = model.note_list.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::SelectFile(index) => {
                let note = &self.note_list[index].note;
                self.note_content = note.read().map_or_else(
                    |err| {
                        self.error = Some(err.to_string());
                        None
                    },
                    |result| Some(result),
                )
            }
        }
    }
}
