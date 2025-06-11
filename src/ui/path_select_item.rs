use gtk::prelude::*;
use relm4::{FactorySender, prelude::*};

use crate::persistence::models::AnyCollection;

#[derive(Debug)]
pub struct PathSelectItem {
    pub(super) collection: Box<dyn AnyCollection>,
}

#[derive(Debug)]
pub enum PathSelectItemMsg {}

#[derive(Debug)]
pub enum PathSelectItemOutput {
    Selected(DynamicIndex),
}

#[relm4::factory(pub)]
impl FactoryComponent for PathSelectItem {
    type Init = Box<dyn AnyCollection>;
    type Input = PathSelectItemMsg;
    type Output = PathSelectItemOutput;
    type CommandOutput = ();
    type ParentWidget = gtk::Box;

    view! {
        gtk::Button {
            set_hexpand: true,

            #[watch]
            set_label: &self.collection.name(),

            connect_clicked[sender, index] => move |_| {
                let _ = sender.output(PathSelectItemOutput::Selected(index.clone()));
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
