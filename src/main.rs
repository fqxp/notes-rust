use std::path::PathBuf;

use notes::{storage::FilesystemStorage, ui::main_window::App};
use relm4::RelmApp;

fn main() -> Result<(), ()> {
    let storage = FilesystemStorage::new(PathBuf::from("/home/frank/notes/persönlich"));

    let app = RelmApp::new("de.fqxp.notes");
    app.run_async::<App>(storage);

    Ok(())
}
