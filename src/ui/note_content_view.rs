use crate::{
    storage::Note,
    ui::{
        note_content_panel::{NoteContentPanel, NoteContentPanelOutput},
        note_editor::NoteEditorOutput,
        note_web_view::NoteWebView,
    },
};
use gtk::prelude::*;
use relm4::{Controller, prelude::*};

use super::{
    note_editor::{NoteEditor, NoteEditorMsg},
    note_web_view::NoteWebViewMsg,
};

#[tracker::track]
pub struct NoteContentView {
    content: Option<String>,
    #[tracker::do_not_track]
    panel: Controller<NoteContentPanel>,
    #[tracker::do_not_track]
    web_view: Controller<NoteWebView>,
    #[tracker::do_not_track]
    editor: Controller<NoteEditor>,
    mode: Mode,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Mode {
    Edit,
    View,
}

#[derive(Debug)]
pub enum NoteContentViewMsg {
    ContentChanged(String),
    LoadNote(Note),
    SetMode(Mode),
}

#[derive(Debug)]
pub enum NoteContentViewOutput {
    Changed,
}

#[relm4::component(pub, async)]
impl AsyncComponent for NoteContentView {
    type Init = ();
    type Input = NoteContentViewMsg;
    type Output = NoteContentViewOutput;
    type CommandOutput = ();

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            match &model.content {
                Some(_) => gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    model.panel.widget(),
                },
                _ => gtk::Box {}
            },

            gtk::Stack {
                add_child = &gtk::Box {
                    set_hexpand: true,
                    append: model.web_view.widget()
                } -> { set_name: "view" },
                add_child = &gtk::Box {
                    set_hexpand: true,
                    append: model.editor.widget()
                } -> { set_name: "edit" },
                add_child = &gtk::Label {
                    set_label: "no note loaded",
                } -> { set_name: "none" },

                #[watch]
                set_visible_child_name: match (&model.mode, &model.content) {
                    (_, None) => "none",
                    (Mode::View, _) => "view",
                    (Mode::Edit, _) => "edit"
                },
            },
        }
    }

    async fn init(
        _: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let panel: Controller<NoteContentPanel> = NoteContentPanel::builder().launch(()).forward(
            sender.input_sender(),
            |msg| match msg {
                NoteContentPanelOutput::SetMode(mode) => NoteContentViewMsg::SetMode(mode),
            },
        );
        let web_view: Controller<NoteWebView> =
            NoteWebView::builder().launch(String::from("")).detach();
        let editor: Controller<NoteEditor> = NoteEditor::builder()
            .launch(String::from(""))
            .forward(sender.input_sender(), |msg| match msg {
                NoteEditorOutput::ContentChanged(text) => NoteContentViewMsg::ContentChanged(text),
            });

        let model = NoteContentView {
            panel,
            web_view,
            editor,
            content: None,
            mode: Mode::View,
            tracker: 0,
        };

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        msg: Self::Input,
        _sender: AsyncComponentSender<NoteContentView>,
        _root: &Self::Root,
    ) {
        match msg {
            NoteContentViewMsg::ContentChanged(text) => {
                self.content = Some(text);
            }
            NoteContentViewMsg::SetMode(mode) => {
                self.mode = mode;
                let content = self.content.clone().unwrap();
                match &self.mode {
                    Mode::Edit => {
                        self.editor.emit(NoteEditorMsg::ChangeContent(content));
                    }
                    Mode::View => {
                        self.web_view.emit(NoteWebViewMsg::ChangeContent(content));
                    }
                }
            }
            NoteContentViewMsg::LoadNote(note) => {
                self.mode = Mode::View;
                self.content = note.read_content().await.map_or_else(
                    |err| {
                        // self.error = Some(err.to_string());
                        println!("{}", err);
                        None
                    },
                    |result| Some(result),
                );
                let content = self.content.clone().unwrap();
                self.web_view.emit(NoteWebViewMsg::ChangeContent(content));
            }
        }
    }
}
