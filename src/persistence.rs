pub mod filesystem;
pub mod models;
pub mod storage;

use filesystem::FilesystemStorage;
use storage::{DynItemStorage, ItemStorage};

use crate::errors::Error;

pub async fn build_storage_from_url(uri: &str) -> Result<Box<dyn ItemStorage>, Error> {
    if uri.starts_with("file://") {
        let fs_storage = FilesystemStorage::from_uri(uri).await?;
        Ok(Box::new(DynItemStorage {
            inner: Box::new(fs_storage),
        }))
    } else {
        Err(Error::UnknownStorageBackend(uri.to_owned()))
    }
}
