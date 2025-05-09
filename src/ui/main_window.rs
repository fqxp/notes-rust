use crate::{
    storage::NoteStorage,
    ui::{
        note_content_view::{NoteContentView, NoteContentViewOutput},
        note_list_view::{NoteListItem, NoteListOutput},
    },
};
use gtk::prelude::*;
use relm4::{RelmListBoxExt, prelude::*};

use super::note_content_view::NoteContentViewInput;

pub enum EditMode {
    EDITING,
    VIEWING,
}

pub struct App {
    note_list: FactoryVecDeque<NoteListItem>,
    current_note_index: Option<usize>,
    mode: EditMode,
    error: Option<String>,
    content_view: AsyncController<NoteContentView>,
}

#[derive(Debug)]
pub enum AppMsg {
    SelectFile(usize),
    ContentChanged,
}

#[relm4::component(pub, async)]
impl AsyncComponent for App {
    type Init = NoteStorage;
    type Input = AppMsg;
    type Output = ();
    type CommandOutput = ();

    view! {
        #[root]
        gtk::Window {
            set_title: Some("notes"),
            set_default_width: 600,
            set_default_height: 400,

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
                    set_end_child = model.content_view.widget(),
                }
            }
        }
    }

    async fn init(
        storage: NoteStorage,
        root: Self::Root,
        sender: AsyncComponentSender<App>,
    ) -> AsyncComponentParts<Self> {
        let content_view: AsyncController<NoteContentView> = NoteContentView::builder()
            .launch(())
            .forward(sender.input_sender(), |msg| match msg {
                NoteContentViewOutput::ContentChanged => AppMsg::ContentChanged,
            });

        let note_list = FactoryVecDeque::builder()
            .launch(gtk::ListBox::default())
            .forward(sender.input_sender(), |output| match output {
                NoteListOutput::SelectFile(index) => AppMsg::SelectFile(index),
            });

        let mut model = App {
            note_list,
            current_note_index: None,
            mode: EditMode::VIEWING,
            error: None,
            content_view,
        };

        let notes = storage
            .list()
            .await
            .map_or_else(
                |err| {
                    model.error = Some(err.to_string());
                    None
                },
                |result| Some(result),
            )
            .unwrap();
        for note in notes.into_iter() {
            model.note_list.guard().push_back(note);
        }

        let note_list_box = model.note_list.widget();
        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        msg: Self::Input,
        _sender: AsyncComponentSender<App>,
        _root: &Self::Root,
    ) {
        match msg {
            AppMsg::SelectFile(index) => {
                self.current_note_index = Some(index);
                let current_note = &self.note_list[index].note;
                self.content_view
                    .sender()
                    .send(NoteContentViewInput::LoadNote(current_note.clone()))
                    .unwrap()
            }
            AppMsg::ContentChanged => {
                println!("content changed");
            }
        }
    }
}
