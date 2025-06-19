use relm4::{Component, ComponentParts};

use adw;
use gtk::{self, prelude::*};

use crate::{icon_names, persistence::models::AnyNote, ui::app::AppMsg};

#[derive(Debug, PartialEq)]
pub enum TitleMode {
    Normal,
    EditTitle,
}

#[derive(Debug)]
pub enum TitleMsg {
    SetCurrentNote(Option<Box<dyn AnyNote>>),
    SetMode(TitleMode),
    RenameNote(String),
}

pub struct Title {
    current_note: Option<Box<dyn AnyNote>>,
    mode: TitleMode,
}

#[relm4::component(pub)]
impl Component for Title {
    type Init = ();
    type Input = TitleMsg;
    type Output = AppMsg;
    type CommandOutput = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,

            adw::WindowTitle {
                #[watch]
                set_title: format!("notes{}",
                    if let Some(current_note) = model.current_note.clone()  {
                        format!(" ‒ {}", current_note.name())
                    } else {
                        "".to_string()
                    }).as_str(),
                #[watch]
                set_visible: model.mode == TitleMode::Normal,
            },
            gtk::Entry {
                #[watch]
                set_text: if let Some(current_note) = model.current_note.clone() {
                        current_note.name()
                    } else {
                        "".to_string()
                    }.as_str(),
                #[watch]
                set_visible: model.mode == TitleMode::EditTitle,

                connect_activate[sender] => move |entry| {
                    let new_name = entry.buffer().text().to_string();
                    let _ = sender.input(TitleMsg::RenameNote(new_name));
                },
            },
            gtk::Button {
                set_icon_name: icon_names::EDIT_REGULAR,
                set_tooltip_text: Some("Rename note"),
                #[watch]
                set_visible: model.current_note.is_some() && model.mode == TitleMode::Normal,

                connect_clicked[sender] => move |_| {
                    let _ = sender.output(AppMsg::StartRenameNote());
                }
            }
        }
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = Self {
            current_note: None,
            mode: TitleMode::Normal,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: relm4::ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            TitleMsg::SetCurrentNote(note) => {
                self.current_note = note;
            }
            TitleMsg::SetMode(mode) => {
                self.mode = mode;
            }
            TitleMsg::RenameNote(new_name) => {
                let _ = sender.output(AppMsg::RenameNote(
                    self.current_note.as_ref().unwrap().clone(),
                    new_name,
                ));
            }
        }

        self.update_view(widgets, sender);
    }
}
