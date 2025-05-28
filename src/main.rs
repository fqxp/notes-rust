mod errors;
mod persistence;
mod ui;
mod util;

use crate::{persistence::build_storage_from_url, ui::window::App};
use relm4::RelmApp;

fn main() -> Result<(), ()> {
    let app = RelmApp::new("de.fqxp.notes");
    let storage = build_storage_from_url("fs:///home/frank/code/notes-rust/sample-notes");

    app.run_async::<App>(storage);

    Ok(())
}
