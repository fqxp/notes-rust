use crate::{
    icon_names,
    persistence::models::{AnyItem, ItemKind},
};
use gtk::prelude::*;
use relm4::{prelude::*, typed_view::list::RelmListItem};

#[derive(Debug)]
pub(super) struct NoteListItem {
    pub(super) item: Box<dyn AnyItem>,
}

impl NoteListItem {
    pub fn from_any_item(item: Box<dyn AnyItem>) -> Self {
        Self { item }
    }
}

impl PartialEq for NoteListItem {
    fn eq(&self, other: &Self) -> bool {
        self.item.name() == other.item.name()
    }
}

pub struct NoteListItemWidgets {
    icon: gtk::Image,
    label: gtk::Label,
}

impl RelmListItem for NoteListItem {
    type Root = gtk::Box;
    type Widgets = NoteListItemWidgets;

    fn setup(_list_item: &gtk::ListItem) -> (Self::Root, Self::Widgets) {
        relm4::view! {
            root = gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 10,
                set_margin_top: 10,
                set_margin_bottom: 10,

                #[name = "icon"]
                gtk::Image {
                    set_icon_name: Some(icon_names::DOCUMENT_ONE_PAGE_REGULAR),
                    set_icon_size: gtk::IconSize::Large,
                },
                #[name = "label"]
                gtk::Label {
                    set_margin_top: 4,
                    set_margin_bottom: 4,
                },
            }
        }

        (root, Self::Widgets { icon, label })
    }

    fn bind(&mut self, widgets: &mut Self::Widgets, _root: &mut Self::Root) {
        widgets.label.set_text(&self.item.name());
        let icon_name = match self.item.kind() {
            ItemKind::Note => icon_names::DOCUMENT_ONE_PAGE_REGULAR,
            ItemKind::Collection => icon_names::FOLDER_REGULAR,
            ItemKind::Attachment => icon_names::IMAGE_REGULAR,
        };
        widgets.icon.set_icon_name(Some(icon_name));
    }
}
