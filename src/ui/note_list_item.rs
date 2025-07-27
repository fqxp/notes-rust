use crate::{
    icon_names,
    persistence::models::{AnyItem, ItemKind},
};
use gtk;
use gtk::prelude::*;
use relm4::{prelude::*, view};

#[derive(Debug)]
pub struct NoteListItem {
    pub item: Box<dyn AnyItem>,
}

pub struct NoteListItemWidgets {
    pub icon: gtk::Image,
    pub label: gtk::Label,
}

impl NoteListItem {
    pub fn from_any_item(any_item: Box<dyn AnyItem>) -> Self {
        Self { item: any_item }
    }

    pub fn setup(_list_item: &gtk::ListItem) -> (gtk::Box, NoteListItemWidgets) {
        view! {
            root = gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 10,
                set_margin_top: 10,
                set_margin_bottom: 10,
                set_focusable: false,

                #[name = "icon"]
                gtk::Image {
                    set_icon_size: gtk::IconSize::Large,
                },
                #[name = "label"]
                gtk::Label {
                    set_margin_top: 4,
                    set_margin_bottom: 4,
                },
            }
        }

        let widgets = NoteListItemWidgets { icon, label };

        (root, widgets)
    }

    pub fn bind(&self, root: &mut gtk::Widget, widgets: &mut NoteListItemWidgets) {
        widgets.label.set_text(&self.item.name());

        let icon_name = match self.item.kind() {
            ItemKind::Note => icon_names::DOCUMENT_ONE_PAGE_REGULAR,
            ItemKind::Collection => icon_names::FOLDER_REGULAR,
            ItemKind::Attachment => icon_names::IMAGE_REGULAR,
        };
        widgets.icon.set_icon_name(Some(icon_name));

        let tooltip_markup = format!(
            "Last changed {}
<i>{}</i>",
            self.item
                .updated_at()
                .format_iso8601()
                .map_or(String::from("unknown"), |gs| gs.to_string())
                .as_str(),
            self.item.location(),
        );
        root.set_tooltip_markup(Some(tooltip_markup.as_str()));
    }
}
