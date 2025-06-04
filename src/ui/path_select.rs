use gtk::prelude::*;
use relm4::{ComponentParts, ComponentSender, FactorySender, SimpleComponent, prelude::*};

use crate::persistence::models::AnyCollection;

#[derive(Debug)]
pub struct PathPart {
    collection: Box<dyn AnyCollection>,
}

#[derive(Debug)]
pub enum PathPartMsg {}

#[derive(Debug)]
pub enum PathPartOutput {
    Selected(DynamicIndex),
}

#[relm4::factory(pub)]
impl FactoryComponent for PathPart {
    type Init = Box<dyn AnyCollection>;
    type Input = PathPartMsg;
    type Output = PathPartOutput;
    type CommandOutput = ();
    type ParentWidget = gtk::Box;

    view! {
        gtk::Button {
            set_hexpand: true,

            #[watch]
            set_label: &self.collection.name(),

            connect_clicked[sender, index] => move |_| {
                let _=sender.output(PathPartOutput::Selected(index.clone()));
            },
        },
    }

    fn init_model(
        collection: Self::Init,
        _index: &Self::Index,
        _sender: FactorySender<Self>,
    ) -> Self {
        Self { collection }
    }
}

pub struct PathSelect {
    path_parts: FactoryVecDeque<PathPart>,
}

#[derive(Debug)]
pub enum PathSelectMsg {
    SetPath(Vec<Box<dyn AnyCollection>>),
    SelectCollectionAt(DynamicIndex),
}

#[derive(Debug)]
pub enum PathSelectOutput {
    SelectedCollectionPath(Vec<Box<dyn AnyCollection>>),
}

#[relm4::component(pub)]
impl SimpleComponent for PathSelect {
    type Init = ();
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
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let path_parts = FactoryVecDeque::builder()
            .launch(gtk::Box::default())
            .forward(sender.input_sender(), |output| match output {
                PathPartOutput::Selected(index) => PathSelectMsg::SelectCollectionAt(index),
            });

        let model = PathSelect { path_parts };

        let path_select_box = model.path_parts.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            PathSelectMsg::SetPath(collections) => {
                let mut guard = self.path_parts.guard();
                guard.clear();
                for collection in collections {
                    guard.push_back(collection);
                }
            }

            PathSelectMsg::SelectCollectionAt(index) => {
                let mut guard = self.path_parts.guard();
                while index.current_index() < guard.len() - 1 {
                    guard.pop_back();
                }
                guard.drop();

                let collections = Vec::from_iter(
                    self.path_parts
                        .iter()
                        .map(|path_part| path_part.collection.clone()),
                );
                let _ = sender.output(PathSelectOutput::SelectedCollectionPath(collections));
            }
        }
    }
}
