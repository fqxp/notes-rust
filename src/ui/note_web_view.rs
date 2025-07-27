use relm4::{Component, ComponentParts, ComponentSender};
use webkit6::{NavigationPolicyDecision, PolicyDecisionType, prelude::*};

use crate::{ui::app::AppMsg, util::markdown::markdown_to_html};

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
    type Output = AppMsg;
    type CommandOutput = ();

    view! {
        gtk::ScrolledWindow {
            set_hexpand: true,
            set_vexpand: true,

            #[local_ref]
            web_view -> webkit6::WebView {
                set_vexpand: true,
                #[track(model.changed(NoteWebView::content()))]
                grab_focus: (),
                #[track(model.changed(NoteWebView::content()))]
                load_html[None]: markdown_to_html(model.get_content()).as_str(),

                connect_decide_policy[sender] => move |_, decision, decision_type| {
                    if decision_type == PolicyDecisionType::NavigationAction {
                        if let Some(nav_decision) = decision.downcast_ref::<NavigationPolicyDecision>(){
                            if let Some(mut nav_action) = nav_decision.navigation_action() {
                                if let Some(request) = nav_action.request() {
                                    let uri = request.uri().expect("uri not to be empty").to_string();

                                    if uri != "about:blank" {
                                        sender.output_sender().emit(AppMsg::ClickedWebLink(uri));

                                        decision.ignore();
                                        return true;
                                    }
                                }
                            }
                        }
                    }

                    false
                }
            },
        }
    }

    fn init(
        content: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let web_view = webkit6::WebView::builder().build();
        let stylesheet = webkit6::UserStyleSheet::new(
            "
            body {
                background-color: #fef1e0;
                margin: 0 20px 0 20px;
            }
            h1, h2, h3, h4, h5, h6 {
                font-family: serif;
            }
            ",
            webkit6::UserContentInjectedFrames::AllFrames,
            webkit6::UserStyleLevel::User,
            &[],
            &[],
        );
        web_view
            .user_content_manager()
            .unwrap()
            .add_style_sheet(&stylesheet);

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
