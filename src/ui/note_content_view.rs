use crate::{
    persistence::models::AnyNote,
    ui::{
        note_content_panel::{NoteContentPanel, NoteContentPanelOutput},
        note_editor::NoteEditorOutput,
        note_web_view::NoteWebView,
    },
};
use gtk::prelude::*;
use relm4::{Controller, prelude::*};

use super::{
    note_content_panel::NoteContentPanelMsg,
    note_editor::{NoteEditor, NoteEditorMsg},
    note_web_view::NoteWebViewMsg,
};

pub struct NoteContentView {
    note: Option<Box<dyn AnyNote>>,
    content: Option<String>,
    mode: Mode,
    panel: Controller<NoteContentPanel>,
    web_view: Controller<NoteWebView>,
    editor: Controller<NoteEditor>,
}

impl NoteContentView {
    fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;

        if let Some(content) = self.content.clone() {
            match &self.mode {
                Mode::Edit => {
                    self.editor.emit(NoteEditorMsg::SetContent {
                        content,
                        name: self.note.as_ref().unwrap().name(),
                    });
                }
                Mode::View => {
                    self.web_view.emit(NoteWebViewMsg::ChangeContent(content));
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Mode {
    Edit,
    View,
}

impl Mode {
    fn toggled(&self) -> Mode {
        match self {
            Mode::Edit => Mode::View,
            Mode::View => Mode::Edit,
        }
    }
}

#[derive(Debug)]
pub enum NoteContentViewMsg {
    ContentChanged(String),
    LoadedNote {
        note: Box<dyn AnyNote>,
        content: String,
    },
    SetMode(Mode),
    ToggleMode(),
}

#[derive(Debug)]
pub enum NoteContentViewOutput {
    ContentChanged {
        note: Box<dyn AnyNote>,
        content: String,
    },
}

#[relm4::component(pub, async)]
impl AsyncComponent for NoteContentView {
    type Init = ();
    type Input = NoteContentViewMsg;
    type Output = NoteContentViewOutput;
    type CommandOutput = ();

    view! {
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
            note: None,
            content: None,
            panel,
            web_view,
            editor,
            mode: Mode::View,
        };

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        msg: Self::Input,
        sender: AsyncComponentSender<NoteContentView>,
        _root: &Self::Root,
    ) {
        match msg {
            NoteContentViewMsg::ContentChanged(content) => {
                self.content = Some(content.clone());
                let _ = sender.output(NoteContentViewOutput::ContentChanged {
                    note: self.note.clone().unwrap().clone(),
                    content,
                });
                // self.etag = self
                //     .note
                //     .unwrap()
                //     .clone_box()
                //     .as_ref()
                //     .save_content(&self.content.clone().unwrap(), &self.etag)
                //     .await
                //     .map_or_else(
                //         |err| {
                //             println!("error while saving: {}", err);
                //             None
                //         },
                //         |etag| Some(etag),
                //     );
            }
            NoteContentViewMsg::SetMode(mode) => {
                self.set_mode(mode);
                self.panel
                    .sender()
                    .emit(NoteContentPanelMsg::SetMode(self.mode.clone()));
            }
            NoteContentViewMsg::ToggleMode() => {
                self.set_mode(self.mode.toggled());
                self.panel
                    .sender()
                    .emit(NoteContentPanelMsg::SetMode(self.mode.clone()));
            }
            NoteContentViewMsg::LoadedNote { note, content } => {
                // (self.content, self.etag) = note.load_content().await.map_or_else(
                //     |err| {
                //         println!("error while loading: {}", err);
                //         (None, None)
                //     },
                //     |(content, etag)| (Some(content), etag),
                // );
                self.note = Some(note);
                self.content = Some(content);
                self.mode = Mode::View;

                let content = self.content.clone().unwrap();
                self.web_view.emit(NoteWebViewMsg::ChangeContent(content));
            }
        }
    }
}
