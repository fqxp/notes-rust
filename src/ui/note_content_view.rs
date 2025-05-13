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
    note: Option<Note>,
    content: Option<String>,
    #[tracker::do_not_track]
    etag: Option<String>,
    mode: Mode,
    language: Option<sourceview5::Language>,
    #[tracker::do_not_track]
    panel: Controller<NoteContentPanel>,
    #[tracker::do_not_track]
    web_view: Controller<NoteWebView>,
    #[tracker::do_not_track]
    editor: Controller<NoteEditor>,
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
    ContentChanged(String),
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
            note: None,
            content: None,
            etag: None,
            language: None,
            panel,
            web_view,
            editor,
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
                self.set_content(Some(text));

                self.etag = self
                    .note
                    .clone()
                    .unwrap()
                    .save_content(&self.content.clone().unwrap(), &self.etag)
                    .await
                    .map_or_else(
                        |err| {
                            println!("error while saving: {}", err);
                            None
                        },
                        |etag| Some(etag),
                    );
            }
            NoteContentViewMsg::SetMode(mode) => {
                self.mode = mode;

                let content = self.content.clone().unwrap();
                match &self.mode {
                    Mode::Edit => {
                        self.editor.emit(NoteEditorMsg::SetContent(
                            content,
                            self.note.clone().unwrap().filename,
                        ));
                    }
                    Mode::View => {
                        self.web_view.emit(NoteWebViewMsg::ChangeContent(content));
                    }
                }
            }
            NoteContentViewMsg::LoadNote(note) => {
                (self.content, self.etag) = note.load_content().await.map_or_else(
                    |err| {
                        println!("error while loading: {}", err);
                        (None, None)
                    },
                    |(content, etag)| (Some(content), etag),
                );
                self.note = Some(note);
                self.mode = Mode::View;

                let content = self.content.clone().unwrap();
                self.web_view.emit(NoteWebViewMsg::ChangeContent(content));
            }
        }
    }
}
