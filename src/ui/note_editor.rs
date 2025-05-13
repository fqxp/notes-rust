use gtk::prelude::*;
use relm4::{Component, ComponentParts, ComponentSender};

#[tracker::track]
pub struct NoteEditor {
    buffer: sourceview5::Buffer,
}

#[derive(Debug)]
pub enum NoteEditorMsg {
    ChangeContent(String),
}

#[derive(Debug)]
pub enum NoteEditorOutput {
    ContentChanged(String),
}

#[relm4::component(pub)]
impl Component for NoteEditor {
    type Init = String;
    type Input = NoteEditorMsg;
    type Output = NoteEditorOutput;
    type CommandOutput = ();

    view! {
        #[root]
        gtk::ScrolledWindow {
            set_hexpand: true,
            set_vexpand: true,
            sourceview5::View {
                 set_buffer: Some(&model.buffer),
            },
        }
    }

    fn init(
        content: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            buffer: sourceview5::Buffer::new(None),
            tracker: 0,
        };

        model.buffer.set_text(content.as_str());

        let sender_clone = sender.clone();
        model.buffer.connect_changed(move |buffer| {
            let text = buffer
                .text(&buffer.start_iter(), &buffer.end_iter(), false)
                .to_string();
            let _ = sender_clone.output(NoteEditorOutput::ContentChanged(text));
        });

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            NoteEditorMsg::ChangeContent(text) => {
                // self.buffer.set_language(
                //     sourceview5::LanguageManager::default()
                //         .guess_language(Some(&note.filename), None)
                //         .as_ref(),
                // );
                self.buffer.set_text(text.as_str());
            }
        }
    }
}
