mod errors;
mod persistence;
mod ui;
mod util;

use crate::ui::app::App;
use relm4::RelmApp;

pub mod icon_names {
    include!(concat!(env!("OUT_DIR"), "/icon_names.rs"));
}

const APP_ID: &str = "de.fqxp.notes";
const APP_NAME: &str = "notes";
const GITHUB_URL: &str = "https://github.com/fqxp/notes-rust";
const VERSION: &str = "0.1";

fn main() -> Result<(), ()> {
    relm4_icons::initialize_icons(icon_names::GRESOURCE_BYTES, icon_names::RESOURCE_PREFIX);

    let url = "fs://./sample-notes";
    let app = RelmApp::new(APP_ID);

    app.run_async::<App>(url.into());

    Ok(())
}
