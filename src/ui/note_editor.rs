use std::path::PathBuf;

use gtk::{glib, prelude::*};
use relm4::{Component, ComponentParts, ComponentSender};
use sourceview5::prelude::*;

#[tracker::track]
pub struct NoteEditor {
    buffer: sourceview5::Buffer,
    buffer_changed_signal: glib::SignalHandlerId,
}

#[derive(Debug)]
pub enum NoteEditorMsg {
    SetContent(String, PathBuf),
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
        let buffer = sourceview5::Buffer::new(None);
        buffer.set_text(content.as_str());
        let sender_clone = sender.clone();

        let buffer_changed_signal = buffer.connect_changed(move |buffer| {
            let text = buffer
                .text(&buffer.start_iter(), &buffer.end_iter(), false)
                .to_string();
            let _ = sender_clone.output(NoteEditorOutput::ContentChanged(text));
        });

        let model = Self {
            buffer,
            buffer_changed_signal,
            tracker: 0,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            NoteEditorMsg::SetContent(text, filename) => {
                self.buffer.block_signal(&self.buffer_changed_signal);
                self.buffer.set_text(text.as_str());
                self.buffer.unblock_signal(&self.buffer_changed_signal);

                self.buffer.set_language(
                    sourceview5::LanguageManager::default()
                        .guess_language(Some(filename.as_path()), None)
                        .as_ref(),
                );
            }
        }
    }
}
