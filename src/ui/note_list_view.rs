use gtk::prelude::*;
use relm4::{RelmListBoxExt, prelude::*};

use crate::persistence::{build_storage_from_url, models::AnyItem};

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
    error: Option<String>,
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
        gtk::ScrolledWindow{
            #[local_ref]
            note_list_box -> gtk::ListBox {
                connect_row_activated[sender] => move |list_box, row| {
                    let index = list_box.index_of_child(row).unwrap() as usize;
                    sender.input_sender().emit(NoteListViewMsg::SelectNode(index));
                }
            },
        }
    }

    async fn init(
        _: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let storage = build_storage_from_url("fs:///home/frank/code/notes-rust/sample-notes");

        let note_list = FactoryVecDeque::builder()
            .launch(gtk::ListBox::default())
            .detach();

        let mut model = Self {
            note_list,
            error: None,
        };

        let notes = storage
            .list_items()
            .map_or_else(
                |err| {
                    model.error = Some(err.to_string());
                    None
                },
                |result| Some(result),
            )
            .unwrap();

        for note in notes {
            model.note_list.guard().push_back(note);
        }

        let note_list_box = model.note_list.widget();

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        msg: Self::Input,
        sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match msg {
            NoteListViewMsg::SelectNode(index) => {
                let maybe_node: Option<Box<dyn AnyItem>> = self.find_node_by_index(index);
                if maybe_node.is_some() {
                    let node = maybe_node.unwrap();
                    let _ = sender.output(NoteListViewOutput::SelectedNode(node.clone_box()));
                }
            }
        }
    }
}
