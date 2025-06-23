mod errors;
mod persistence;
mod ui;
mod util;

use std::{
    env,
    ffi::OsString,
    path::{Path, PathBuf},
};

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

    let notes_rootdir =
        PathBuf::from(env::var_os("NOTES_ROOTDIR").unwrap_or("./sample-notes".into()))
            .canonicalize()
            .unwrap();
    let uri = format!("file://{}", notes_rootdir.to_string_lossy());
    let app = RelmApp::new(APP_ID);

    app.run_async::<App>(uri.into());

    Ok(())
}
