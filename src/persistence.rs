pub mod filesystem;
pub mod models;
pub mod storage;

use std::{path::PathBuf, str::FromStr};

use filesystem::FileSystemStorage;
use storage::{DynItemStorage, ItemStorage};

pub fn build_storage_from_url(url: &str) -> Box<dyn ItemStorage> {
    if url.starts_with("fs://") {
        let root = PathBuf::from_str(url.strip_prefix("fs://").unwrap()).unwrap();
        let fs_storage = Box::new(FileSystemStorage::new(&root));
        Box::new(DynItemStorage { inner: fs_storage })
    } else {
        panic!("unknown storage URL {}", url);
    }
}
