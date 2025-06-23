use std::{fmt, string::FromUtf8Error};

use gtk::glib;

#[derive(Debug)]
pub enum Error {
    DecodeError(FromUtf8Error),
    DoesNotExist { uri: String },
    IoError(glib::Error),
    OtherError(String),
    UnknownStorageBackend(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::DecodeError(err) => write!(f, "{}", err.to_string()),
            Error::DoesNotExist { uri } => write!(f, "could not find {}", uri),
            Error::IoError(err) => write!(f, "{}", err.to_string()),
            Error::OtherError(msg) => write!(f, "{}", msg),
            Error::UnknownStorageBackend(err) => write!(f, "{}", err.to_string()),
        }
    }
}

impl From<glib::Error> for Error {
    fn from(err: glib::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Error {
        Error::DecodeError(err)
    }
}

impl From<(Vec<u8>, glib::Error)> for Error {
    fn from(err: (Vec<u8>, glib::Error)) -> Error {
        Error::IoError(err.1)
    }
}
