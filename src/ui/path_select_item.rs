use gtk::prelude::ButtonExt;
use relm4::prelude::*;

use crate::persistence::models::AnyCollection;

#[derive(Debug)]
pub struct PathSelectItem {
    pub(super) collection: Box<dyn AnyCollection>,
}

#[derive(Debug)]
pub enum PathSelectItemOutput {
    Selected,
}

#[relm4::component(pub)]
impl Component for PathSelectItem {
    type Init = Box<dyn AnyCollection>;
    type Input = ();
    type Output = PathSelectItemOutput;
    type CommandOutput = ();

    view! {
        gtk::LinkButton {
            #[watch]
            set_label: &model.collection.name(),

            connect_clicked[sender] => move |_| {
                let _ = sender.output(PathSelectItemOutput::Selected);
            },
        },
    }

    fn init(
        collection: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            collection: collection.clone(),
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
