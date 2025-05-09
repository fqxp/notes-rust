use crate::{storage::Note, util::markdown::markdown_to_html};
use gtk::prelude::*;
use relm4::prelude::*;
use webkit6::prelude::WebViewExt;

#[tracker::track]
pub struct NoteContentView {
    content: Option<String>,
}

#[derive(Debug)]
pub enum NoteContentViewInput {
    LoadNote(Note),
}

#[derive(Debug)]
pub enum NoteContentViewOutput {
    ContentChanged,
}

#[relm4::component(pub, async)]
impl AsyncComponent for NoteContentView {
    type Init = ();
    type Input = NoteContentViewInput;
    type Output = NoteContentViewOutput;
    type CommandOutput = ();

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            match &model.content {
                Some(markdown) => &webkit6::WebView {
                   set_vexpand: true,
                   #[watch]
                   load_html[None]: markdown_to_html(markdown).as_str(),
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
        _sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let model = NoteContentView {
            content: None,
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
            // NoteContentViewInput::LoadedContent(content) => {
            //     self.content = Some(content);
            // }
            // NoteContentViewInput::UnloadedContent => {
            //     self.content = None;
            // }
            NoteContentViewInput::LoadNote(note) => {
                self.content = note.read_content().await.map_or_else(
                    |err| {
                        // self.error = Some(err.to_string());
                        println!("{}", err);
                        None
                    },
                    |result| Some(result),
                )
            }
        }
    }
}
