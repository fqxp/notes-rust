use crate::persistence::models::{AnyItem, AnyNote, ItemKind};
use crate::persistence::storage::{ItemStorage, NoteContent};
use crate::ui::note_content_view::{NoteContentView, NoteContentViewMsg};
use crate::ui::sidebar::{Sidebar, SidebarOutput};
use adw;
use gtk::prelude::*;
use relm4::actions::{AccelsPlus, RelmAction, RelmActionGroup};
use relm4::{main_application, prelude::*};

use super::note_content_view::NoteContentViewOutput;
use super::sidebar::SidebarMsg;

relm4::new_action_group!(pub AppActions, "app");
relm4::new_stateless_action!(pub QuitAction, AppActions, "quit");
relm4::new_stateless_action!(pub FocusSearchEntryAction, AppActions, "focus-search-entry");
relm4::new_stateless_action!(pub ToggleModeAction, AppActions, "toggle");

pub struct App {
    storage: Box<dyn ItemStorage>,
    error: Option<String>,
    sidebar: AsyncController<Sidebar>,
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

        self.sidebar
            .sender()
            .emit(SidebarMsg::UpdateNoteList(notes));
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
        #[name = "root"]
        adw::Window {
            set_default_width: 600,
            set_default_height: 400,

            gtk::Box{
                set_orientation: gtk::Orientation::Vertical,
                adw::HeaderBar {
                    #[wrap(Some)]
                    set_title_widget = &adw::WindowTitle {
                        set_title: "notes",
                    },
                    set_show_end_title_buttons: false,
                },
                gtk::Paned::new(gtk::Orientation::Horizontal) {
                    set_position: 250,
                    set_wide_handle: true,

                    #[wrap(Some)]
                    set_start_child = model.sidebar.widget() ,

                    #[wrap(Some)]
                    set_end_child = model.content_view.widget(),
                }
            }
        }
    }

    async fn init(
        storage_url: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let storage = build_storage_from_url(storage_url.clone().as_str())
            .ok()
            .unwrap();
        let content_view: AsyncController<NoteContentView> = NoteContentView::builder()
            .launch(())
            .forward(sender.input_sender(), |msg| -> Self::Input {
                match msg {
                    NoteContentViewOutput::ContentChanged { note, content } => {
                        AppMsg::ContentChanged { note, content }
                    }
                }
            });
        let sidebar: AsyncController<Sidebar> =
            Sidebar::builder()
                .launch(())
                .forward(sender.input_sender(), |msg| -> Self::Input {
                    match msg {
                        SidebarOutput::SelectedNode(note) => AppMsg::SelectedNode(note),
                    }
                });

        let model = App {
            storage,
            error: None,
            sidebar,
            content_view,
        };

        let widgets = view_output!();

        model.update_note_list().await;

        // setup actions

        let mut group = RelmActionGroup::<AppActions>::new();
        let sender_clone = model.sidebar.sender().clone();
        let focus_search_entry_action: RelmAction<FocusSearchEntryAction> =
            RelmAction::new_stateless(move |_| sender_clone.emit(SidebarMsg::FocusSearchEntry()));
        group.add_action(focus_search_entry_action);

        let sender_clone = model.content_view.sender().clone();
        let toggle_action: RelmAction<ToggleModeAction> = RelmAction::new_stateless(move |_| {
            sender_clone.emit(NoteContentViewMsg::ToggleMode());
        });
        group.add_action(toggle_action);

        let quit_action: RelmAction<QuitAction> = RelmAction::new_stateless(move |_| {
            main_application().quit();
        });
        group.add_action(quit_action);

        group.register_for_widget(&widgets.root);

        let app = main_application();
        app.set_accelerators_for_action::<ToggleModeAction>(&["<Control>Return"]);
        app.set_accelerators_for_action::<FocusSearchEntryAction>(&["<Control>K"]);
        app.set_accelerators_for_action::<QuitAction>(&["<Control>Q"]);

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
