use gtk::{
    gio::{self, prelude::*},
    glib,
};
use std::{fmt, path::PathBuf, string::FromUtf8Error};

#[derive(Clone, Debug, PartialEq)]
pub struct Note {
    pub filename: PathBuf,
    file: gio::File,
}

impl Note {
    pub fn new_from_file(file: gio::File) -> Self {
        let filename = file.path().unwrap();

        Self { filename, file }
    }

    pub fn name(self: &Self) -> String {
        self.file.basename().unwrap().display().to_string()
    }

    pub fn display_filename(self: &Self) -> String {
        let base_filename = self
            .file
            .basename()
            .unwrap()
            .to_string_lossy()
            .rsplit("/")
            .next()
            .unwrap()
            .to_string();

        match base_filename.strip_suffix(".md") {
            Some(name) => name.to_string(),
            None => base_filename,
        }
    }

    pub fn file(self: &Self) -> gio::File {
        self.file.clone()
    }

    pub async fn load_content(&self) -> Result<(String, Option<String>), ReadError> {
        let (content, etag) = self.file.load_contents_future().await?;
        let etag = etag.and_then(|g_string| Some(g_string.to_string()));
        println!("load_content etag={:?}", &etag);

        return Result::Ok((String::from_utf8(content.to_vec())?, etag));
    }

    pub async fn save_content(
        &self,
        content: &String,
        etag: &Option<String>,
    ) -> Result<String, WriteError> {
        let (_, etag_after_save) = self
            .file
            .replace_contents_future(
                content.as_bytes().to_vec(),
                etag.as_deref(),
                false,
                gio::FileCreateFlags::NONE,
            )
            .await?;
        println!("save_content etag={:?}", &etag);

        Result::Ok(etag_after_save.to_string())
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

impl From<(Vec<u8>, glib::Error)> for WriteError {
    fn from(err: (Vec<u8>, glib::Error)) -> WriteError {
        WriteError::IoError(err.1)
    }
}

impl fmt::Display for WriteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WriteError::IoError(err) => write!(f, "{}", err.to_string()),
        }
    }
}

#[derive(Clone)]
pub struct NoteStorage {
    basedir: gio::File,
}

impl NoteStorage {
    pub fn new(basedir: PathBuf) -> Self {
        Self {
            basedir: gio::File::for_path(basedir),
        }
    }

    pub async fn list(self: Self) -> Result<Vec<Note>, gtk::glib::Error> {
        let file_infos = self
            .basedir
            .enumerate_children_future("", gio::FileQueryInfoFlags::NONE, glib::Priority::DEFAULT)
            .await?;

        let result: Vec<Note> = file_infos
            .map(|file_info| Note::new_from_file(self.basedir.child(file_info.unwrap().name())))
            .collect();

        Result::Ok(result)
    }
}
