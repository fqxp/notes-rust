use gtk::prelude::*;
use relm4::{Component, ComponentParts, ComponentSender};
use webkit6::prelude::*;

use crate::util::markdown::markdown_to_html;

#[tracker::track]
pub struct NoteWebView {
    content: String,
}

#[derive(Debug)]
pub enum NoteWebViewMsg {
    ChangeContent(String),
}

impl NoteWebView {}

#[relm4::component(pub)]
impl Component for NoteWebView {
    type Init = String;
    type Input = NoteWebViewMsg;
    type Output = ();
    type CommandOutput = ();

    view! {
        gtk::ScrolledWindow {
            set_hexpand: true,
            set_vexpand: true,
            #[name = "web_view"]
            webkit6::WebView {
                set_vexpand: true,
                #[track(model.changed(NoteWebView::content()))]
                grab_focus: (),
                #[track(model.changed(NoteWebView::content()))]
                load_html[None]: markdown_to_html(model.get_content()).as_str(),
            },
        }
    }

    fn init(
        content: Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = NoteWebView {
            content,
            tracker: 0,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(
        &mut self,
        msg: Self::Input,
        _sender: ComponentSender<NoteWebView>,
        _root: &Self::Root,
    ) {
        self.reset();

        match msg {
            NoteWebViewMsg::ChangeContent(content) => {
                self.set_content(content);
            }
        }
    }
}
