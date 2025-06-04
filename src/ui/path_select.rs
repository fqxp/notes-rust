use gtk::prelude::*;
use relm4::{ComponentParts, ComponentSender, SimpleComponent, prelude::*};

use crate::{
    persistence::models::CollectionPath,
    ui::path_select_item::{PathSelectItem, PathSelectItemOutput},
};

impl From<&FactoryVecDeque<PathSelectItem>> for CollectionPath {
    fn from(path_select_items: &FactoryVecDeque<PathSelectItem>) -> Self {
        let mut collection_path = CollectionPath::default();
        for path_select_item in path_select_items.iter() {
            collection_path.push(path_select_item.collection.clone());
        }
        collection_path
    }
}

pub struct PathSelect {
    path_select_items: FactoryVecDeque<PathSelectItem>,
}

#[derive(Debug)]
pub enum PathSelectMsg {
    SetCollectionPath(CollectionPath),
    SelectCollectionAt(DynamicIndex),
}

#[derive(Debug)]
pub enum PathSelectOutput {
    SelectedCollectionPath(CollectionPath),
}

#[relm4::component(pub)]
impl SimpleComponent for PathSelect {
    type Init = CollectionPath;
    type Input = PathSelectMsg;
    type Output = PathSelectOutput;

    view! {
        root = gtk::Box{
            #[local_ref]
            path_select_box -> gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 4,
            }
        }
    }

    fn init(
        collection_path: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let path_select_items = FactoryVecDeque::builder()
            .launch(gtk::Box::default())
            .forward(sender.input_sender(), |output| match output {
                PathSelectItemOutput::Selected(index) => PathSelectMsg::SelectCollectionAt(index),
            });

        let model = PathSelect { path_select_items };

        let path_select_box = model.path_select_items.widget();

        let widgets = view_output!();

        sender.input(PathSelectMsg::SetCollectionPath(collection_path.clone()));

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            PathSelectMsg::SetCollectionPath(collection_path) => {
                let mut guard = self.path_select_items.guard();
                guard.clear();
                for collection in collection_path.iter() {
                    guard.push_back(collection.clone());
                }
            }

            PathSelectMsg::SelectCollectionAt(index) => {
                {
                    let mut guard = self.path_select_items.guard();
                    while index.current_index() < guard.len() - 1 {
                        guard.pop_back();
                    }
                }

                let _ = sender.output(PathSelectOutput::SelectedCollectionPath(
                    CollectionPath::from(&self.path_select_items),
                ));
            }
        }
    }
}
