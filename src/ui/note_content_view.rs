use crate::{
    storage::Note,
    ui::note_content_panel::{NoteContentPanel, NoteContentPanelOutput},
    util::markdown::markdown_to_html,
};
use gtk::prelude::*;
use relm4::{Controller, prelude::*};
use sourceview5::{self, prelude::*};
use webkit6::prelude::WebViewExt;

#[tracker::track]
pub struct NoteContentView {
    content: Option<String>,
    #[tracker::do_not_track]
    panel: Controller<NoteContentPanel>,
    mode: Mode,
    buffer: sourceview5::Buffer,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Mode {
    Edit,
    View,
}

#[derive(Debug)]
pub enum NoteContentViewMsg {
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

            container_add = match &model.content {
                Some(_) => gtk::Box {
                    append = model.panel.widget(),
                },
                _ => gtk::Box {}
            },

            match &model.content {
                Some(markdown) => &gtk::Stack {
                    #[watch]
                    set_visible_child_name: match &model.mode {
                        Mode::View => "view",
                        Mode::Edit => "edit"
                    },
                    add_child = &webkit6::WebView {
                       set_vexpand: true,
                       #[watch]
                       load_html[None]: markdown_to_html(markdown).as_str(),
                    } -> web_view: gtk::StackPage { set_name: "view" },
                    add_child = &sourceview5::View::with_buffer(&model.buffer) {
                    } -> editor: gtk::StackPage { set_name: "edit"},
                }
                None => {
                    &gtk::Label {
                        set_label: "no note loaded",
                    }
                }
            }
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
        let model = NoteContentView {
            panel,
            content: None,
            mode: Mode::View,
            buffer: sourceview5::Buffer::new(None),
            tracker: 0,
        };

        let sender_clone = sender.clone();
        model.buffer.connect_changed(move |_| {
            let _ = sender_clone.output(NoteContentViewOutput::Changed);
        });

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
            NoteContentViewMsg::SetMode(mode) => {
                self.mode = mode;
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
                self.buffer.set_language(
                    sourceview5::LanguageManager::default()
                        .guess_language(Some(&note.filename), None)
                        .as_ref(),
                );
                self.buffer.set_text(self.content.as_ref().unwrap());
            }
        }
    }
}
