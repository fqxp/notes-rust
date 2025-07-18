use std::path::PathBuf;

use gtk::{glib, prelude::*};
use relm4::{Component, ComponentParts, ComponentSender};
use sourceview5::prelude::*;

use super::app::AppMsg;

pub struct NoteEditor {
    buffer: sourceview5::Buffer,
    buffer_changed_signal: glib::SignalHandlerId,
}

#[derive(Debug)]
pub enum NoteEditorMsg {
    SetContent { content: String, name: String },
}

#[relm4::component(pub)]
impl Component for NoteEditor {
    type Init = String;
    type Input = NoteEditorMsg;
    type Output = AppMsg;
    type CommandOutput = ();

    view! {
        #[root]
        gtk::ScrolledWindow {
            set_hexpand: true,
            set_vexpand: true,
            sourceview5::View {
                set_buffer: Some(&model.buffer),
                set_monospace: true,
                set_highlight_current_line: true,
                set_auto_indent: true,
                set_indent_width: 2,
                set_indent_on_tab: true,
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
            let content = buffer
                .text(&buffer.start_iter(), &buffer.end_iter(), false)
                .to_string();
            let _ = sender_clone.output(AppMsg::NoteContentChanged(content));
        });

        let model = Self {
            buffer,
            buffer_changed_signal,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            NoteEditorMsg::SetContent { content, name } => {
                self.buffer.block_signal(&self.buffer_changed_signal);
                self.buffer.set_text(content.as_str());
                self.buffer.unblock_signal(&self.buffer_changed_signal);

                self.buffer.set_language(
                    sourceview5::LanguageManager::default()
                        .guess_language(Some(PathBuf::from(name)), None)
                        .as_ref(),
                );
            }
        }
    }
}
