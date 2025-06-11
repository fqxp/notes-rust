use std::{cell::Ref, convert::identity};

use crate::{
    persistence::models::{AnyItem, CollectionPath},
    ui::{
        note_list_item::{NoteListItem, NoteListItemWidgets},
        path_select::PathSelectMsg,
    },
};
use gtk::glib::{self};
use gtk::{
    gio::{self},
    prelude::*,
};
use relm4::prelude::*;

use super::{app::AppMsg, path_select::PathSelect};

#[derive(Debug, Clone)]
pub enum SortOrder {
    AToZ,
    ZToA,
    OldestFirst,
    NewestFirst,
}

#[derive(Debug)]
struct SortOption<'a> {
    order: SortOrder,
    title: &'a str,
}

const SORT_OPTIONS: [SortOption; 4] = [
    SortOption {
        order: SortOrder::AToZ,
        title: "A → Z",
    },
    SortOption {
        order: SortOrder::ZToA,
        title: "Z → A",
    },
    SortOption {
        order: SortOrder::OldestFirst,
        title: "oldest first",
    },
    SortOption {
        order: SortOrder::NewestFirst,
        title: "newest first",
    },
];

pub struct Sidebar {
    note_list_store: gio::ListStore,
    note_list_model: gtk::SingleSelection,
    note_filter_list_model: gtk::FilterListModel,
    note_sort_list_model: gtk::SortListModel,
    path_select: Controller<PathSelect>,
}

impl Sidebar {
    fn build_sorter(&self, sort_order: SortOrder) -> impl IsA<gtk::Sorter> {
        gtk::CustomSorter::new(move |lhs, rhs| {
            let lhs_note_list_item: Ref<NoteListItem> =
                lhs.downcast_ref::<glib::BoxedAnyObject>().unwrap().borrow();
            let rhs_note_list_item: Ref<NoteListItem> =
                rhs.downcast_ref::<glib::BoxedAnyObject>().unwrap().borrow();

            match sort_order {
                SortOrder::AToZ => lhs_note_list_item
                    .item
                    .name()
                    .cmp(&rhs_note_list_item.item.name())
                    .into(),
                SortOrder::ZToA => rhs_note_list_item
                    .item
                    .name()
                    .cmp(&lhs_note_list_item.item.name())
                    .into(),
                SortOrder::OldestFirst => {
                    let lhs_updated_at = lhs_note_list_item.item.updated_at();
                    let rhs_updated_at = rhs_note_list_item.item.updated_at();

                    lhs_updated_at.cmp(&rhs_updated_at).into()
                }
                SortOrder::NewestFirst => {
                    let lhs_updated_at = lhs_note_list_item.item.updated_at();
                    let rhs_updated_at = rhs_note_list_item.item.updated_at();

                    rhs_updated_at.cmp(&lhs_updated_at).into()
                }
            }
        })
    }

    fn build_filter(&self, search_term: String) -> impl IsA<gtk::Filter> {
        gtk::CustomFilter::new(move |list_item| {
            if search_term.len() == 0 {
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
                .contains(search_term.to_lowercase().as_str())
        })
    }
}

#[derive(Debug)]
pub enum SidebarMsg {
    SelectedItem(u32),
    UpdateNoteList(Vec<Box<dyn AnyItem>>),
    FocusSearchEntry(),
    ChangeSearchTerm(String),
    ChangeSorting(SortOrder),
    SetCollectionPath(CollectionPath),
}

#[relm4::component(pub, async)]
impl AsyncComponent for Sidebar {
    type Init = CollectionPath;
    type Input = SidebarMsg;
    type Output = AppMsg;
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

            #[name = "sort_dropdown"]
            gtk::DropDown{
                set_model: Some(&sort_options),
                set_expression: Some(&sort_expression),

                connect_selected_notify[sender] => move |dropdown: &gtk::DropDown| {
                    let obj = dropdown.selected_item().unwrap();
                    let sort_option: Ref<SortOption> = obj.downcast_ref::<glib::BoxedAnyObject>().unwrap().borrow();
                    let _ = sender
                        .input_sender()
                        .emit(SidebarMsg::ChangeSorting(sort_option.order.clone()));
                }
            },

            append = model.path_select.widget(),

            gtk::ScrolledWindow {
                set_vexpand: true,

                #[name = "list_view"]
                gtk::ListView {
                    set_factory: Some(&factory),
                    set_model: Some(&model.note_list_model),

                    connect_activate[sender] => move |_, index| {
                        sender.input_sender().emit(SidebarMsg::SelectedItem(index));
                    }
                },
            }
        }
    }

    async fn init(
        collection_path: Self::Init,
        _root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
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

        // note list
        let note_list_store = gio::ListStore::new::<glib::BoxedAnyObject>();
        let note_filter_list_model =
            gtk::FilterListModel::new(Some(note_list_store.clone()), None::<gtk::Filter>);
        let note_sort_list_model =
            gtk::SortListModel::new(Some(note_filter_list_model.clone()), None::<gtk::Sorter>);
        let note_list_model = gtk::SingleSelection::new(Some(note_sort_list_model.clone()));

        let path_select: Controller<PathSelect> = PathSelect::builder()
            .launch(collection_path.clone())
            .forward(sender.output_sender(), identity);

        let model = Self {
            note_list_store,
            note_filter_list_model,
            note_sort_list_model,
            note_list_model,
            path_select,
        };

        let mut sort_options = gio::ListStore::new::<glib::BoxedAnyObject>();
        sort_options.extend(SORT_OPTIONS.map(move |sc| glib::BoxedAnyObject::new(sc)));
        let sort_expression = gtk::ClosureExpression::new::<String>(
            &[] as &[gtk::Expression],
            glib::closure!(|sc: glib::BoxedAnyObject| {
                let sort_option: Ref<SortOption> = sc.borrow();
                sort_option.title
            }),
        );

        let widgets = view_output!();

        sender.input(SidebarMsg::ChangeSorting(SortOrder::AToZ));

        AsyncComponentParts { model, widgets }
    }

    async fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        msg: Self::Input,
        sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        use SidebarMsg::*;

        match msg {
            SelectedItem(index) => {
                if let Some(list_item) = self.note_list_model.item(index) {
                    let item: Ref<NoteListItem> = list_item
                        .downcast_ref::<glib::BoxedAnyObject>()
                        .unwrap()
                        .borrow();
                    let _ = sender.output(AppMsg::SelectedItem(item.item.clone()));
                }
            }
            UpdateNoteList(items) => {
                self.note_list_store.remove_all();
                self.note_list_store.extend(items.iter().map(|item| {
                    glib::BoxedAnyObject::new(NoteListItem::from_any_item(item.clone()))
                }));
            }
            FocusSearchEntry() => {
                widgets.search_entry.grab_focus();
            }
            ChangeSearchTerm(search_term) => {
                let filter = self.build_filter(search_term);
                self.note_filter_list_model.set_filter(Some(&filter));
            }
            ChangeSorting(sort_order) => {
                let sorter = self.build_sorter(sort_order);
                self.note_sort_list_model.set_sorter(Some(&sorter));
            }
            SetCollectionPath(collection_path) => {
                self.path_select
                    .emit(PathSelectMsg::SetCollectionPath(collection_path));
            }
        }
    }
}
