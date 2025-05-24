use std::{fmt, string::FromUtf8Error};

use gtk::glib;

pub enum ReadError {
    IoError(glib::Error),
    DecodeError(FromUtf8Error),
    NotAFile,
    MismatchedStorageBackend,
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReadError::IoError(err) => write!(f, "{}", err.to_string()),
            ReadError::DecodeError(err) => write!(f, "{}", err.to_string()),
            ReadError::NotAFile => write!(f, "not a file"),
            ReadError::MismatchedStorageBackend => write!(f, "mismatched storage backend"),
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
    NotAFile,
    MismatchedStorageBackend,
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
            WriteError::NotAFile => write!(f, "not a file"),
            WriteError::MismatchedStorageBackend => write!(f, "mismatched storage backend"),
        }
    }
}
