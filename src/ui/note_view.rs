use std::convert::identity;

use crate::{
    persistence::models::AnyNote,
    ui::{note_panel::NotePanel, note_web_view::NoteWebView},
};
use gtk::prelude::*;
use relm4::{Controller, prelude::*};

use super::{
    app::AppMsg,
    note_editor::{NoteEditor, NoteEditorMsg},
    note_panel::NotePanelMsg,
    note_web_view::NoteWebViewMsg,
};

pub struct NoteView {
    note: Option<Box<dyn AnyNote>>,
    content: Option<String>,
    mode: Mode,
    panel: Controller<NotePanel>,
    web_view: Controller<NoteWebView>,
    editor: Controller<NoteEditor>,
}

impl NoteView {
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
    pub fn toggled(&self) -> Mode {
        match self {
            Mode::Edit => Mode::View,
            Mode::View => Mode::Edit,
        }
    }
}

#[derive(Debug)]
pub enum NoteViewMsg {
    ContentChanged(String),
    LoadedNote {
        note: Box<dyn AnyNote>,
        content: String,
    },
    SetMode(Mode),
}

#[relm4::component(pub, async)]
impl AsyncComponent for NoteView {
    type Init = ();
    type Input = NoteViewMsg;
    type Output = AppMsg;
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
        let panel: Controller<NotePanel> = NotePanel::builder()
            .launch(())
            .forward(sender.output_sender(), identity);
        let web_view: Controller<NoteWebView> = NoteWebView::builder()
            .launch(String::from(""))
            .forward(sender.output_sender(), identity);
        let editor: Controller<NoteEditor> = NoteEditor::builder()
            .launch(String::from(""))
            .forward(sender.output_sender(), identity);
        let model = NoteView {
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
        sender: AsyncComponentSender<NoteView>,
        _root: &Self::Root,
    ) {
        match msg {
            NoteViewMsg::ContentChanged(content) => {
                self.content = Some(content.clone());
                let _ = sender.output(AppMsg::ContentChanged {
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
            NoteViewMsg::SetMode(mode) => {
                self.set_mode(mode);
                self.panel
                    .sender()
                    .emit(NotePanelMsg::SetMode(self.mode.clone()));
            }
            NoteViewMsg::LoadedNote { note, content } => {
                self.note = Some(note);
                self.content = Some(content);
                self.mode = Mode::View;

                let content = self.content.clone().unwrap();
                self.web_view.emit(NoteWebViewMsg::ChangeContent(content));
            }
        }
    }
}
