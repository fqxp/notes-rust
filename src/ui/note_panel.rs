use gtk::prelude::*;
use relm4::Component;
use relm4::prelude::*;

use crate::icon_names;
use crate::ui::note_view::Mode;

use super::app::AppMsg;

pub struct NotePanel {
    mode: Mode,
}

#[derive(Debug)]
pub enum NotePanelMsg {
    SetMode(Mode),
}

#[relm4::component(pub)]
impl Component for NotePanel {
    type Init = ();
    type Input = NotePanelMsg;
    type Output = AppMsg;
    type CommandOutput = ();

    view! {
        #[root]
        gtk::Box{
            set_orientation: gtk::Orientation::Horizontal,
            set_homogeneous: true,
            add_css_class: "linked",


            #[name="view_toggle_button"]
            gtk::ToggleButton {
                set_can_focus: false,
                #[watch]
                set_active: model.mode == Mode::View,
                connect_toggled[sender] => move |btn| {
                    if btn.is_active() {
                        let _ = sender.output(AppMsg::SetMode(Mode::View));
                    }
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 8,
                    gtk::Image {
                        set_icon_name: Some(icon_names::DOCUMENT_ONE_PAGE_REGULAR)
                    },
                    gtk::Label {
                        set_label: "View",
                    }
                }
            },
            gtk::ToggleButton {
                set_can_focus: false,
                #[watch]
                set_active: model.mode == Mode::Edit,
                set_group: Some(&view_toggle_button),
                connect_toggled[sender] => move |btn| {
                    if btn.is_active() {
                        let _ = sender.output(AppMsg::SetMode(Mode::Edit));
                    }
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 8,
                    gtk::Image {
                        set_icon_name: Some(icon_names::EDIT_REGULAR),
                        set_margin_end: 4,
                    },
                    gtk::Label {
                        set_label: "Edit",
                    }
                }
            },
        }
    }

    fn init(
        _: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = NotePanel { mode: Mode::View };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(
        &mut self,
        msg: Self::Input,
        _sender: ComponentSender<NotePanel>,
        _root: &Self::Root,
    ) {
        match msg {
            NotePanelMsg::SetMode(mode) => {
                self.mode = mode;
            }
        }
    }
}
