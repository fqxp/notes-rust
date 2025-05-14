use crate::storage::Note;
use crate::ui::note_content_view::{NoteContentView, NoteContentViewMsg};
use crate::ui::note_list_view::{NoteListView, NoteListViewOutput};
use adw;
use gtk::prelude::*;
use relm4::actions::AccelsPlus;
use relm4::prelude::*;

use super::note_content_view::ToggleModeAction;

pub struct App {
    error: Option<String>,
    list_view: AsyncController<NoteListView>,
    content_view: AsyncController<NoteContentView>,
}

#[derive(Debug)]
pub enum AppMsg {
    SelectedNote(Note),
    ContentChanged,
}

#[relm4::component(pub, async)]
impl AsyncComponent for App {
    type Init = ();
    type Input = AppMsg;
    type Output = ();
    type CommandOutput = ();

    view! {
        #[root]
        adw::Window {
            set_title: Some("notes"),
            set_default_width: 600,
            set_default_height: 400,

            gtk::Box{
                set_orientation: gtk::Orientation::Vertical,
                gtk::Label {
                    #[watch]
                    set_label: model.error.as_deref().unwrap_or(""),
                },
                gtk::Paned::new(gtk::Orientation::Horizontal) {
                    set_position: 250,
                    set_wide_handle: true,

                    #[wrap(Some)]
                    set_start_child = model.list_view.widget() ,

                    #[wrap(Some)]
                    set_end_child = model.content_view.widget(),
                }
            }
        }
    }

    async fn init(
        _: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let content_view: AsyncController<NoteContentView> =
            NoteContentView::builder().launch(()).detach();
        let list_view: AsyncController<NoteListView> = NoteListView::builder().launch(()).forward(
            sender.input_sender(),
            |msg| -> Self::Input {
                match msg {
                    NoteListViewOutput::SelectedNote(note) => AppMsg::SelectedNote(note),
                }
            },
        );

        let model = App {
            error: None,
            list_view,
            content_view,
        };

        let widgets = view_output!();

        let app = relm4::main_application();
        app.set_accelerators_for_action::<ToggleModeAction>(&["<Control>Return"]);

        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        msg: Self::Input,
        _sender: AsyncComponentSender<App>,
        _root: &Self::Root,
    ) {
        match msg {
            AppMsg::SelectedNote(note) => {
                self.content_view.emit(NoteContentViewMsg::LoadNote(note))
            }
            AppMsg::ContentChanged => {
                println!("content changed");
            }
        }
    }
}
