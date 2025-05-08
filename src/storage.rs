use gtk::gio;
use gtk::gio::prelude::*;
use gtk::glib;
use std::path::PathBuf;
use std::{fmt, string::FromUtf8Error};

pub struct Note {
    pub name: String,
    pub filename: PathBuf,
}

impl Note {
    pub fn new_from_file(file: gio::File) -> Self {
        let name = file.basename().unwrap().display().to_string();
        let filename = file.path().unwrap();
        Self { name, filename }
    }
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

pub enum WriteError {
    IoError(glib::Error),
}

impl From<glib::Error> for WriteError {
    fn from(err: glib::Error) -> WriteError {
        WriteError::IoError(err)
    }
}

#[derive(Clone)]
pub struct NoteStorage {
    basedir: PathBuf,
}

impl NoteStorage {
    pub fn new(basedir: PathBuf) -> Self {
        Self { basedir }
    }

    pub async fn list(self: Self) -> Result<Vec<Note>, gtk::glib::Error> {
        let basedir = gio::File::for_path(self.basedir);
        let file_infos = basedir
            .enumerate_children_future("", gio::FileQueryInfoFlags::NONE, glib::Priority::DEFAULT)
            .await?;

        let result: Vec<Note> = file_infos
            .map(|file_info| Note::new_from_file(basedir.child(file_info.unwrap().name())))
            .collect();

        Result::Ok(result)
    }

    pub async fn read_content(self: &Self, note: &Note) -> Result<String, ReadError> {
        let (content, _etag) = gio::File::for_path(&note.filename)
            .load_contents_future()
            .await?;

        return Result::Ok(String::from_utf8(content.to_vec())?);
    }

    // pub fn write_content(note: Note) -> Result<(), WriteError> {}
}
