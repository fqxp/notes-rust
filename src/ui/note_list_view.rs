use gtk::prelude::*;
use relm4::{RelmListBoxExt, prelude::*};

use crate::persistence::models::AnyItem;

struct NoteListItem {
    item: Box<dyn AnyItem>,
}

#[relm4::factory(pub)]
impl FactoryComponent for NoteListItem {
    type Init = Box<dyn AnyItem>;
    type Input = ();
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = gtk::ListBox;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 10,
            set_margin_top: 10,
            set_margin_bottom: 10,

            #[name(open_button)]
            gtk::Label {
                set_margin_top: 4,
                set_margin_bottom: 4,
                #[watch]
                set_label: &self.item.name(),
            },
        }
    }

    fn init_model(
        init: Self::Init,
        _index: &Self::Index,
        _sender: relm4::FactorySender<Self>,
    ) -> Self {
        Self { item: init }
    }
}

pub struct NoteListView {
    note_list: FactoryVecDeque<NoteListItem>,
}

impl NoteListView {
    fn find_node_by_index(&self, index: usize) -> Option<Box<dyn AnyItem>> {
        self.note_list
            .get(index)
            .map(|note_list_item| note_list_item.item.clone_box())
    }
}

#[derive(Debug)]
pub enum NoteListViewMsg {
    SelectNode(usize),
    UpdatedNoteList(Vec<Box<dyn AnyItem>>),
    FocusSearchEntry(),
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
            },
            gtk::ScrolledWindow {
                set_vexpand: true,
                #[local_ref]
                note_list_box -> gtk::ListBox {
                    connect_row_activated[sender] => move |list_box, row| {
                        let index = list_box.index_of_child(row).unwrap() as usize;
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
        let note_list = FactoryVecDeque::builder()
            .launch(gtk::ListBox::default())
            .detach();
        let model = Self { note_list };
        let note_list_box = model.note_list.widget();

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
                let maybe_node: Option<Box<dyn AnyItem>> = self.find_node_by_index(index);
                if maybe_node.is_some() {
                    let node = maybe_node.unwrap();
                    let _ = sender.output(NoteListViewOutput::SelectedNode(node.clone_box()));
                }
            }
            UpdatedNoteList(notes) => {
                self.note_list.guard().clear();
                for note in notes {
                    self.note_list.guard().push_back(note);
                }
            }
            FocusSearchEntry() => {
                widgets.search_entry.grab_focus();
            }
        }
    }
}
