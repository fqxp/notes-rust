use gtk::prelude::*;
use relm4::Component;
use relm4::prelude::*;

use crate::ui::note_content_view::Mode;

pub struct NoteContentPanel {
    mode: Mode,
}

#[derive(Debug)]
pub enum NoteContentPanelMsg {
    SetMode(Mode),
}

#[derive(Debug)]
pub enum NoteContentPanelOutput {
    SetMode(Mode),
}

#[relm4::component(pub)]
impl Component for NoteContentPanel {
    type Init = ();
    type Input = NoteContentPanelMsg;
    type Output = NoteContentPanelOutput;
    type CommandOutput = ();

    view! {
        #[root]
        gtk::Box{
            set_orientation: gtk::Orientation::Horizontal,
            set_homogeneous: true,
            add_css_class: "linked",

            #[name="view_toggle_button"]
            gtk::ToggleButton {
                set_label: "View",
                #[watch]
                set_active: model.mode == Mode::View,
                connect_toggled[sender] => move |btn| {
                    if btn.is_active() {
                        let _ = sender.output(NoteContentPanelOutput::SetMode(Mode::View));
                    }
                },
            },
            gtk::ToggleButton {
                set_label: "Edit",
                #[watch]
                set_active: model.mode == Mode::Edit,
                connect_toggled[sender] => move |btn| {
                    if btn.is_active() {
                        let _ = sender.output(NoteContentPanelOutput::SetMode(Mode::Edit));
                    }
                },
            },
        }
    }

    fn init(
        _: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = NoteContentPanel { mode: Mode::View };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(
        &mut self,
        msg: Self::Input,
        _sender: ComponentSender<NoteContentPanel>,
        _root: &Self::Root,
    ) {
        match msg {
            NoteContentPanelMsg::SetMode(mode) => {
                self.mode = mode;
            }
        }
    }
}
