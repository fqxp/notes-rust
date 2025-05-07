use gtk::gio::prelude::{FileExt, FileExtManual};
use gtk::gio::{self, Cancellable};
use gtk::glib;
use std::path::{Path, PathBuf};
use std::{fmt, string::FromUtf8Error};

pub struct Note {
    pub name: String,
    pub file: gio::File,
}

pub enum ReadError {
    IoError(glib::Error),
    DecodeError(FromUtf8Error),
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReadError::IoError(err) => write!(f, "{}", err.to_string()),
            ReadError::DecodeError(err) => write!(f, "{}", err.to_string()),
        }
    }
}

impl From<glib::Error> for ReadError {
    fn from(err: glib::Error) -> ReadError {
        ReadError::IoError(err)
    }
}

impl From<FromUtf8Error> for ReadError {
    fn from(err: FromUtf8Error) -> ReadError {
        ReadError::DecodeError(err)
    }
}

impl Note {
    pub fn new(file: gio::File) -> Self {
        let name = file.basename().unwrap().display().to_string();
        Self { name, file }
    }

    pub fn read(self: &Self) -> Result<String, ReadError> {
        let (contents, _etag) = self
            .file
            .load_contents((None as Option<Cancellable>).as_ref())?;

        return Result::Ok(String::from_utf8(contents.to_vec())?);
    }
}
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
