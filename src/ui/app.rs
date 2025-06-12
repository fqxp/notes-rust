use std::convert::identity;

use crate::icon_names;
use crate::persistence::build_storage_from_url;
use crate::persistence::models::{AnyItem, AnyNote, CollectionPath, ItemKind};
use crate::persistence::storage::{ItemStorage, NoteContent};
use crate::ui::note_view::{NoteView, NoteViewMsg};
use crate::ui::sidebar::Sidebar;
use adw;
use gtk::prelude::*;
use relm4::actions::{AccelsPlus, RelmAction, RelmActionGroup};
use relm4::{main_application, prelude::*};

use super::about_dialog::{AboutDialog, AboutDialogMsg};
use super::note_view::Mode;
use super::sidebar::SidebarMsg;

relm4::new_action_group!(pub AppActions, "app");
relm4::new_stateless_action!(pub AboutAction, AppActions, "about");
relm4::new_stateless_action!(pub FocusSearchEntryAction, AppActions, "focus-search-entry");
relm4::new_stateless_action!(pub QuitAction, AppActions, "quit");
relm4::new_stateless_action!(pub ToggleModeAction, AppActions, "toggle");
relm4::new_stateless_action!(pub UpAction, AppActions, "up");

pub struct App {
    about_dialog_controller: Controller<AboutDialog>,
    storage: Box<dyn ItemStorage>,
    sidebar: AsyncController<Sidebar>,
    note_view: AsyncController<NoteView>,
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
    ShowAboutDialog(),
    Up(),
}

#[relm4::component(pub, async)]
impl AsyncComponent for App {
    type Init = String;
    type Input = AppMsg;
    type Output = ();
    type CommandOutput = ();

    view! {
        #[name = "root"]
        adw::ApplicationWindow {
            set_default_width: 600,
            set_default_height: 400,

            gtk::Box{
                set_orientation: gtk::Orientation::Vertical,
                adw::HeaderBar {
                    #[wrap(Some)]
                    set_title_widget = &adw::WindowTitle {
                        set_title: "notes",
                    },

                    pack_end = &gtk::MenuButton {
                        set_icon_name: icon_names::MENU,
                        set_popover: Some(&gtk::PopoverMenu::from_model(Some(&main_menu))),
                    },
                },

                gtk::Paned::new(gtk::Orientation::Horizontal) {
                    set_position: 250,
                    set_wide_handle: true,

                    #[wrap(Some)]
                    set_start_child = model.sidebar.widget() ,

                    #[wrap(Some)]
                    set_end_child = model.note_view.widget(),
                }
            }
        }
    }

    menu! {
        main_menu: {
            "About" => AboutAction,
            section! {
                "Quit" => QuitAction,
            },
        }
    }

    async fn init(
        storage_url: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let about_dialog_controller: Controller<AboutDialog> =
            AboutDialog::builder().launch(()).detach();
        let storage = build_storage_from_url(storage_url.clone().as_str())
            .ok()
            .unwrap();
        let current_path =
            CollectionPath::from(storage.root().await.expect("valid root collection"));

        let note_view: AsyncController<NoteView> = NoteView::builder()
            .launch(())
            .forward(sender.input_sender(), identity);
        let sidebar: AsyncController<Sidebar> = Sidebar::builder()
            .launch(current_path.clone())
            .forward(sender.input_sender(), identity);

        let model = App {
            about_dialog_controller,
            storage,
            sidebar,
            note_view,
            current_path,
            mode: Mode::View,
        };

        let widgets = view_output!();

        // setup actions

        let mut group = RelmActionGroup::<AppActions>::new();

        let sender_clone = sender.clone();
        let about_action: RelmAction<AboutAction> = RelmAction::new_stateless(move |_| {
            sender_clone.input(AppMsg::ShowAboutDialog());
        });
        group.add_action(about_action);

        let sender_clone = model.sidebar.sender().clone();
        let focus_search_entry_action: RelmAction<FocusSearchEntryAction> =
            RelmAction::new_stateless(move |_| sender_clone.emit(SidebarMsg::FocusSearchEntry()));
        group.add_action(focus_search_entry_action);

        let sender_clone = sender.clone();
        let toggle_action: RelmAction<ToggleModeAction> = RelmAction::new_stateless(move |_| {
            sender_clone.input(AppMsg::ToggleMode());
        });
        group.add_action(toggle_action);

        let sender_clone = sender.clone();
        let up_action: RelmAction<UpAction> = RelmAction::new_stateless(move |_| {
            sender_clone.input(AppMsg::Up());
        });
        group.add_action(up_action);

        let quit_action: RelmAction<QuitAction> = RelmAction::new_stateless(move |_| {
            main_application().quit();
        });
        group.add_action(quit_action);

        group.register_for_widget(&widgets.root);

        let app = main_application();
        app.set_accelerators_for_action::<FocusSearchEntryAction>(&["<Control>K"]);
        app.set_accelerators_for_action::<QuitAction>(&["<Control>Q"]);
        app.set_accelerators_for_action::<ToggleModeAction>(&["<Control>Return"]);
        app.set_accelerators_for_action::<UpAction>(&["<Control>Up"]);

        sender.input(AppMsg::UpdateItemList());

        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        msg: Self::Input,
        sender: AsyncComponentSender<App>,
        root: &Self::Root,
    ) {
        match msg {
            AppMsg::SelectedCollectionPath(collection_path) => {
                self.current_path = collection_path;
                self.sidebar
                    .emit(SidebarMsg::SetCollectionPath(self.current_path.clone()));
                sender.input(AppMsg::UpdateItemList());
            }
            AppMsg::Up() => {
                if let Some(parent) = self.current_path.parent() {
                    sender.input(AppMsg::SelectedCollectionPath(parent));
                }
            }
            AppMsg::SelectedItem(item) => match item.kind() {
                ItemKind::Note => {
                    let note = item.as_note().expect("note");
                    let result = self.storage.as_ref().load_content(&*note).await;
                    if let Ok(content) = result {
                        self.note_view.emit(NoteViewMsg::LoadedNote {
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
                self.note_view.emit(NoteViewMsg::SetMode(self.mode.clone()));
            }
            AppMsg::ShowAboutDialog() => {
                self.about_dialog_controller
                    .emit(AboutDialogMsg::Show(root.clone()));
            }
            AppMsg::ToggleMode() => {
                self.mode = self.mode.toggled();
                self.note_view.emit(NoteViewMsg::SetMode(self.mode.clone()));
            }
            AppMsg::NoteContentChanged(content) => {
                self.note_view.emit(NoteViewMsg::ContentChanged(content));
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
