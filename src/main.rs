use std::path::PathBuf;

use notes::{storage::NoteStorage, ui::window::App};
use relm4::RelmApp;

fn main() -> Result<(), ()> {
    let storage = NoteStorage::new(PathBuf::from("/home/frank/code/notes-rust/sample-notes"));

    let app = RelmApp::new("de.fqxp.notes");
    app.run_async::<App>(storage);

    Ok(())
}
