use gtk::gio::{self, Cancellable, prelude::FileExt};
use std::path::{Path, PathBuf};

use crate::types::Note;

pub struct FilesystemStorage<'a> {
    basedir: &'a Path,
}

impl<'a> FilesystemStorage<'a> {
    pub fn new(basedir: &'a Path) -> Self {
        Self { basedir }
    }

    pub fn list(self: Self) -> Result<Vec<Note>, gtk::glib::Error> {
        let basedir = gio::File::for_path(self.basedir);
        let file_infos = basedir.enumerate_children(
            "",
            gio::FileQueryInfoFlags::NONE,
            (None as Option<Cancellable>).as_ref(),
        )?;

        let result: Vec<Note> = file_infos
            .map(|file_info| Note::new(basedir.child(file_info.unwrap().name())))
            .collect();

        Result::Ok(result)
    }

    pub fn open(self: Self, relative_path: PathBuf) -> gio::File {
        let absolute_path = self.basedir.join(relative_path);

        gio::File::for_path(absolute_path)
    }
}
