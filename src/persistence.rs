pub mod filesystem;
pub mod models;
pub mod storage;

use std::{path::PathBuf, str::FromStr};

use filesystem::FilesystemStorage;
use storage::{DynItemStorage, ItemStorage};

use crate::errors::Error;

pub fn build_storage_from_url(url: &str) -> Result<Box<dyn ItemStorage>, Error> {
    if url.starts_with("fs://") {
        let root = PathBuf::from_str(url.strip_prefix("fs://").unwrap()).unwrap();
        let fs_storage = Box::new(FilesystemStorage::new(&root));
        Ok(Box::new(DynItemStorage { inner: fs_storage }))
    } else {
        Err(Error::UnknownStorageBackend(url.to_owned()))
    }
}
