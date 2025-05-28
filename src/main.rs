mod errors;
mod persistence;
mod ui;
mod util;

use crate::{persistence::build_storage_from_url, ui::window::App};
use relm4::RelmApp;

pub mod icon_names {
    include!(concat!(env!("OUT_DIR"), "/icon_names.rs"));
}

fn main() -> Result<(), ()> {
    relm4_icons::initialize_icons(icon_names::GRESOURCE_BYTES, icon_names::RESOURCE_PREFIX);

    let app = RelmApp::new("de.fqxp.notes");
    let storage = build_storage_from_url("fs:///home/frank/code/notes-rust/sample-notes");

    app.run_async::<App>(storage);

    Ok(())
}
