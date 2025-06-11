mod errors;
mod persistence;
mod ui;
mod util;

use crate::ui::window::App;
use relm4::RelmApp;

pub mod icon_names {
    include!(concat!(env!("OUT_DIR"), "/icon_names.rs"));
}

fn main() -> Result<(), ()> {
    relm4_icons::initialize_icons(icon_names::GRESOURCE_BYTES, icon_names::RESOURCE_PREFIX);

    let url = "fs://./sample-notes";
    let app = RelmApp::new("de.fqxp.notes");

    app.run_async::<App>(url.into());

    Ok(())
}
