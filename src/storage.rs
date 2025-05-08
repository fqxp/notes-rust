use gtk::gio;
use gtk::gio::prelude::*;
use gtk::glib;
use std::path::PathBuf;
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
        let (contents, _etag) = self.file.load_contents(gio::Cancellable::NONE)?;

        return Result::Ok(String::from_utf8(contents.to_vec())?);
    }
}

#[derive(Clone)]
pub struct FilesystemStorage {
    basedir: PathBuf,
}

impl FilesystemStorage {
    pub fn new(basedir: PathBuf) -> Self {
        Self { basedir }
    }

    pub fn list(self: Self) -> Result<Vec<Note>, gtk::glib::Error> {
        let basedir = gio::File::for_path(self.basedir);
        let file_infos = basedir.enumerate_children(
            "",
            gio::FileQueryInfoFlags::NONE,
            gio::Cancellable::NONE,
        )?;

        let result: Vec<Note> = file_infos
            .map(|file_info| Note::new(basedir.child(file_info.unwrap().name())))
            .collect();

        Result::Ok(result)
    }
}
