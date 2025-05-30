use std::cell::Ref;

use crate::{
    persistence::models::AnyItem,
    ui::note_list_item::{NoteListItem, NoteListItemWidgets},
};
use gtk::{
    gio::{self},
    glib,
    prelude::*,
};
use relm4::prelude::*;

pub struct NoteListView {
    store: gio::ListStore,
    list_model: gtk::SingleSelection,
    filter_list_model: gtk::FilterListModel,
    search_term: String,
}

impl NoteListView {}

#[derive(Debug)]
pub enum NoteListViewMsg {
    SelectNode(u32),
    UpdateNoteList(Vec<Box<dyn AnyItem>>),
    FocusSearchEntry(),
    ChangeSearchTerm(String),
}

#[derive(Debug)]
pub enum NoteListViewOutput {
    SelectedNode(Box<dyn AnyItem>),
}

#[relm4::component(pub, async)]
impl AsyncComponent for NoteListView {
    type Init = ();
    type Input = NoteListViewMsg;
    type Output = NoteListViewOutput;
    type CommandOutput = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_vexpand: true,

            #[name = "search_entry"]
            gtk::Entry {
                set_placeholder_text: Some("Enter search term"),
                set_hexpand: true,
                connect_changed[sender] => move |entry| {
                    let search_term = entry.buffer().text().to_string();
                    let _ = sender.input(Self::Input::ChangeSearchTerm(search_term));
                },
            },
            gtk::ScrolledWindow {
                set_vexpand: true,

                #[name = "list_view"]
                gtk::ListView {
                    set_factory: Some(&factory),
                    set_model: Some(&model.list_model),

                    connect_activate[sender] => move |_, index| {
                        sender.input_sender().emit(NoteListViewMsg::SelectNode(index));
                    }
                },
            }
        }
    }

    async fn init(
        _: Self::Init,
        _root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let store = gio::ListStore::new::<glib::BoxedAnyObject>();
        let filter_list_model =
            gtk::FilterListModel::new(Some(store.clone()), Some(gtk::CustomFilter::new(|_| true)));
        let list_model = gtk::SingleSelection::new(Some(filter_list_model.clone()));
        let factory = gtk::SignalListItemFactory::new();

        factory.connect_setup(move |_factory, list_item| {
            let list_item: &gtk::ListItem = list_item
                .downcast_ref::<gtk::ListItem>()
                .expect("Must be a gtk::ListItem");

            let (root, widgets) = NoteListItem::setup(list_item);

            unsafe {
                root.set_data("widgets", widgets);
            }
            list_item.set_child(Some(&root));
        });

        factory.connect_bind(move |_factory, list_item| {
            let list_item: &gtk::ListItem = list_item
                .downcast_ref::<gtk::ListItem>()
                .expect("Must be a gtk::ListItem");
            let obj = list_item.item().unwrap();
            let note_list_item: Ref<NoteListItem> =
                obj.downcast_ref::<glib::BoxedAnyObject>().unwrap().borrow();

            let root = list_item.child().unwrap();
            let mut widgets: NoteListItemWidgets = unsafe { root.steal_data("widgets") }.unwrap();
            note_list_item.bind(&mut widgets);
            unsafe {
                root.set_data("widgets", widgets);
            }
        });

        let model = Self {
            store,
            filter_list_model,
            list_model,
            search_term: String::from(""),
        };

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        msg: Self::Input,
        sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        use NoteListViewMsg::*;

        match msg {
            SelectNode(index) => {
                if let Some(list_item) = self.store.item(index) {
                    let item: Ref<NoteListItem> = list_item
                        .downcast_ref::<glib::BoxedAnyObject>()
                        .unwrap()
                        .borrow();
                    let _ = sender.output(NoteListViewOutput::SelectedNode(item.item.clone()));
                }
            }
            UpdateNoteList(items) => {
                self.store.remove_all();
                self.store.extend(items.iter().map(|item| {
                    glib::BoxedAnyObject::new(NoteListItem::from_any_item(item.clone()))
                }));
            }
            FocusSearchEntry() => {
                widgets.search_entry.grab_focus();
            }
            ChangeSearchTerm(search_term) => {
                self.search_term = search_term;

                let search_term_clone = self.search_term.clone();
                let filter = gtk::CustomFilter::new(move |list_item| {
                    if search_term_clone.len() == 0 {
                        return true;
                    }

                    let note_list_item: Ref<NoteListItem> = list_item
                        .downcast_ref::<glib::BoxedAnyObject>()
                        .unwrap()
                        .borrow();
                    note_list_item
                        .item
                        .name()
                        .to_lowercase()
                        .contains(search_term_clone.to_lowercase().as_str())
                });

                self.filter_list_model.set_filter(Some(&filter));
            }
        }
    }
}
