use std::path::PathBuf;

use notes::{storage::NoteStorage, ui::main_window::App};
use relm4::RelmApp;

fn main() -> Result<(), ()> {
    let storage = NoteStorage::new(PathBuf::from("/home/frank/notes/persönlich"));

    let app = RelmApp::new("de.fqxp.notes");
    app.run_async::<App>(storage);

    Ok(())
}
