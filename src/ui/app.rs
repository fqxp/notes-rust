use std::convert::identity;

use crate::persistence::build_storage_from_url;
use crate::persistence::models::{AnyItem, AnyNote, CollectionPath, ItemKind};
use crate::persistence::storage::{ItemStorage, NoteContent};
use crate::ui::note_content_view::{NoteContentView, NoteContentViewMsg};
use crate::ui::sidebar::Sidebar;
use adw;
use gtk::prelude::*;
use relm4::actions::{AccelsPlus, RelmAction, RelmActionGroup};
use relm4::{main_application, prelude::*};

use super::note_content_view::Mode;
use super::sidebar::SidebarMsg;

relm4::new_action_group!(pub AppActions, "app");
relm4::new_stateless_action!(pub QuitAction, AppActions, "quit");
relm4::new_stateless_action!(pub FocusSearchEntryAction, AppActions, "focus-search-entry");
relm4::new_stateless_action!(pub ToggleModeAction, AppActions, "toggle");

pub struct App {
    storage: Box<dyn ItemStorage>,
    sidebar: AsyncController<Sidebar>,
    content_view: AsyncController<NoteContentView>,
    current_path: CollectionPath,
    mode: Mode,
}

impl App {
    async fn update_note_list(&self, collection_path: &CollectionPath) {
        let notes = self
            .storage
            .as_ref()
            .list_items(collection_path)
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
    SelectedCollectionPath(CollectionPath),
    SelectedItem(Box<dyn AnyItem>),
    ContentChanged {
        note: Box<dyn AnyNote>,
        content: String,
    },
    UpdateItemList(),
    SetMode(Mode),
    ToggleMode(),
    NoteContentChanged(String),
}

#[relm4::component(pub, async)]
impl AsyncComponent for App {
    type Init = String;
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
        let current_path =
            CollectionPath::from(storage.root().await.expect("valid root collection"));

        let content_view: AsyncController<NoteContentView> = NoteContentView::builder()
            .launch(())
            .forward(sender.input_sender(), identity);
        let sidebar: AsyncController<Sidebar> = Sidebar::builder()
            .launch(current_path.clone())
            .forward(sender.input_sender(), identity);

        let model = App {
            storage,
            sidebar,
            content_view,
            current_path,
            mode: Mode::View,
        };

        let widgets = view_output!();

        // setup actions

        let mut group = RelmActionGroup::<AppActions>::new();
        let sender_clone = model.sidebar.sender().clone();
        let focus_search_entry_action: RelmAction<FocusSearchEntryAction> =
            RelmAction::new_stateless(move |_| sender_clone.emit(SidebarMsg::FocusSearchEntry()));
        group.add_action(focus_search_entry_action);

        let sender_clone = sender.clone();
        let toggle_action: RelmAction<ToggleModeAction> = RelmAction::new_stateless(move |_| {
            sender_clone.input(AppMsg::ToggleMode());
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

        sender.input(AppMsg::UpdateItemList());

        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        msg: Self::Input,
        sender: AsyncComponentSender<App>,
        _root: &Self::Root,
    ) {
        match msg {
            AppMsg::SelectedCollectionPath(collection_path) => {
                self.current_path = collection_path;
                self.sidebar
                    .emit(SidebarMsg::SetCollectionPath(self.current_path.clone()));
                sender.input(AppMsg::UpdateItemList());
            }
            AppMsg::SelectedItem(item) => match item.kind() {
                ItemKind::Note => {
                    let note = item.as_note().expect("note");
                    let result = self.storage.as_ref().load_content(&*note).await;
                    if let Ok(content) = result {
                        self.content_view.emit(NoteContentViewMsg::LoadedNote {
                            note,
                            content: content.content,
                        });
                    } else {
                        panic!(
                            "tried to load content from non-note {:?}: {:?}",
                            note,
                            result.err()
                        );
                    }
                }
                ItemKind::Collection => {
                    let collection = item.as_collection().expect("collection");
                    self.current_path.push(collection);
                    self.sidebar
                        .emit(SidebarMsg::SetCollectionPath(self.current_path.clone()));
                    sender.input(AppMsg::UpdateItemList());
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
            AppMsg::UpdateItemList() => {
                self.update_note_list(&self.current_path).await;
            }
            AppMsg::SetMode(mode) => {
                self.mode = mode;
                self.content_view
                    .emit(NoteContentViewMsg::SetMode(self.mode.clone()));
            }
            AppMsg::ToggleMode() => {
                self.mode = self.mode.toggled();
                self.content_view
                    .emit(NoteContentViewMsg::SetMode(self.mode.clone()));
            }
            AppMsg::NoteContentChanged(content) => {
                self.content_view
                    .emit(NoteContentViewMsg::ContentChanged(content));
                // self.etag = self
                //     .note
                //     .unwrap()
                //     .clone_box()
                //     .as_ref()
                //     .save_content(&self.content.clone().unwrap(), &self.etag)
                //     .await
                //     .map_or_else(
                //         |err| {
                //             println!("error while saving: {}", err);
                //             None
                //         },
                //         |etag| Some(etag),
                //     );
            }
        }
    }
}
