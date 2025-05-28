use crate::{persistence::models::AnyItem, ui::note_list_item::NoteListItem};
use gtk::prelude::*;
use relm4::{prelude::*, typed_view::list::TypedListView};

pub struct NoteListView {
    note_list_view_wrapper: TypedListView<NoteListItem, gtk::SingleSelection>,
    search_term: String,
}

impl NoteListView {}

#[derive(Debug)]
pub enum NoteListViewMsg {
    SelectNode(u32),
    UpdatedNoteList(Vec<Box<dyn AnyItem>>),
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

                #[local_ref]
                note_list_view -> gtk::ListView {
                    connect_activate[sender] => move |_, index| {
                        sender.input_sender().emit(NoteListViewMsg::SelectNode(index));
                    }
                },
            }
        }
    }

    async fn init(
        _: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let note_list_view_wrapper: TypedListView<NoteListItem, gtk::SingleSelection> =
            TypedListView::new();

        let model = Self {
            note_list_view_wrapper,
            search_term: String::from(""),
        };

        let note_list_view = &model.note_list_view_wrapper.view;

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
                if let Some(list_item) = self.note_list_view_wrapper.get(index) {
                    let item: Box<dyn AnyItem> = list_item.borrow().item.clone();
                    let _ = sender.output(NoteListViewOutput::SelectedNode(item));
                }
            }
            UpdatedNoteList(items) => {
                self.note_list_view_wrapper.clear();
                self.note_list_view_wrapper.extend_from_iter(
                    items
                        .iter()
                        .map(|item| NoteListItem::from_any_item(item.clone())),
                );
            }
            FocusSearchEntry() => {
                widgets.search_entry.grab_focus();
            }
            ChangeSearchTerm(search_term) => {
                self.search_term = search_term;
                self.note_list_view_wrapper.clear_filters();
                let search_term_clone = self.search_term.clone();
                self.note_list_view_wrapper.add_filter(move |item| {
                    item.item
                        .name()
                        .to_lowercase()
                        .contains(search_term_clone.to_lowercase().as_str())
                });
            }
        }
    }
}
