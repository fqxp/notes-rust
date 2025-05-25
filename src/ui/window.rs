use crate::persistence::models::{AnyItem, AnyNote, ItemKind};
use crate::persistence::storage::{ItemStorage, NoteContent};
use crate::ui::note_content_view::{NoteContentView, NoteContentViewMsg};
use crate::ui::note_list_view::{NoteListView, NoteListViewOutput};
use adw;
use gtk::prelude::*;
use relm4::actions::AccelsPlus;
use relm4::prelude::*;

use super::note_content_view::{NoteContentViewOutput, ToggleModeAction};
use super::note_list_view::NoteListViewMsg;

pub struct App {
    storage: Box<dyn ItemStorage>,
    error: Option<String>,
    list_view: AsyncController<NoteListView>,
    content_view: AsyncController<NoteContentView>,
}

impl App {
    async fn update_note_list(&self) {
        let notes = self
            .storage
            .as_ref()
            .list_items()
            .await
            .map_or_else(
                |err| {
                    panic!("error loading note list {:?}", err.to_string());
                    // self.model.error = Some(err.to_string());
                },
                |result| Some(result),
            )
            .unwrap();

        self.list_view
            .sender()
            .emit(NoteListViewMsg::UpdatedNoteList(notes));
    }
}

#[derive(Debug)]
pub enum AppMsg {
    SelectedNode(Box<dyn AnyItem>),
    ContentChanged {
        note: Box<dyn AnyNote>,
        content: String,
    },
}

#[relm4::component(pub, async)]
impl AsyncComponent for App {
    type Init = Box<dyn ItemStorage + 'static>;
    type Input = AppMsg;
    type Output = ();
    type CommandOutput = ();

    view! {
        #[root]
        adw::Window {
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
                    set_position: 250,
                    set_wide_handle: true,

                    #[wrap(Some)]
                    set_start_child = model.list_view.widget() ,

                    #[wrap(Some)]
                    set_end_child = model.content_view.widget(),
                }
            }
        }
    }

    async fn init(
        storage: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let content_view: AsyncController<NoteContentView> = NoteContentView::builder()
            .launch(())
            .forward(sender.input_sender(), |msg| -> Self::Input {
                match msg {
                    NoteContentViewOutput::ContentChanged { note, content } => {
                        AppMsg::ContentChanged { note, content }
                    }
                }
            });
        let list_view: AsyncController<NoteListView> = NoteListView::builder().launch(()).forward(
            sender.input_sender(),
            |msg| -> Self::Input {
                match msg {
                    NoteListViewOutput::SelectedNode(note) => AppMsg::SelectedNode(note),
                }
            },
        );

        let model = App {
            storage,
            error: None,
            list_view,
            content_view,
        };

        let widgets = view_output!();

        model.update_note_list().await;

        let app = relm4::main_application();
        app.set_accelerators_for_action::<ToggleModeAction>(&["<Control>Return"]);

        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        msg: Self::Input,
        _sender: AsyncComponentSender<App>,
        _root: &Self::Root,
    ) {
        match msg {
            AppMsg::SelectedNode(note) => match note.kind() {
                ItemKind::Note => {
                    let note = note.as_note().unwrap();
                    let result = self.storage.as_ref().load_content(&*note).await;
                    if let Ok(content) = result {
                        self.content_view.emit(NoteContentViewMsg::LoadedNote {
                            note,
                            content: content.content,
                        })
                    } else {
                        panic!("tried to load content from non-note {:?}", note);
                    }
                }
                _ => {}
            },
            AppMsg::ContentChanged { note, content } => {
                let _ = self
                    .storage
                    .as_ref()
                    .save_content(
                        note.as_ref(),
                        &NoteContent {
                            content,
                            etag: None,
                        },
                    )
                    .await;
            }
        }
    }
}
