use adw::prelude::*;
use relm4::{ComponentParts, ComponentSender, prelude::*};

use crate::persistence::models::{AnyCollection, CollectionPath};

use super::{
    app::AppMsg,
    path_select_item::{PathSelectItem, PathSelectItemOutput},
};

pub struct PathSelect {
    path: CollectionPath,
}

#[derive(Debug)]
pub enum PathSelectMsg {
    SetCollectionPath(CollectionPath),
}

#[relm4::component(pub)]
impl Component for PathSelect {
    type Init = CollectionPath;
    type Input = PathSelectMsg;
    type Output = AppMsg;
    type CommandOutput = ();

    view! {
        root = gtk::Box {
            set_spacing: 4,

            #[name = "path_box"]
            adw::WrapBox {

            },
        }
    }

    fn init(
        path: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = PathSelect { path };

        let widgets = view_output!();

        sender.input(PathSelectMsg::SetCollectionPath(model.path.clone()));

        ComponentParts { model, widgets }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            PathSelectMsg::SetCollectionPath(path) => {
                while let Some(child) = widgets.path_box.last_child() {
                    widgets.path_box.remove(&child);
                }

                let mut collections_so_far: Vec<Box<dyn AnyCollection>> = vec![];
                for (i, collection) in path.iter().enumerate() {
                    if i > 0 {
                        widgets
                            .path_box
                            .append(&gtk::Label::builder().label("→").build());
                    }

                    collections_so_far.push(collection.clone());
                    let collection_path = CollectionPath::from(collections_so_far.clone());

                    let path_select_item = PathSelectItem::builder()
                        .launch(collection.clone())
                        .forward(sender.output_sender(), move |msg| match msg {
                            PathSelectItemOutput::Selected => {
                                AppMsg::SelectedCollectionPath(collection_path.clone())
                            }
                        });
                    widgets.path_box.append(path_select_item.widget());
                }
            }
        }
    }
}
