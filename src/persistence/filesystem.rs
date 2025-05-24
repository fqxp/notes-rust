use std::path::PathBuf;

use crate::errors::{ReadError, WriteError};

use super::{
    models::{AnyItem, Attachment, Collection, Note, StorageBackend},
    storage::TypedItemStorage,
};

pub struct FileSystem;

impl StorageBackend for FileSystem {
    type NoteMeta = PathBuf;
    type CollectionMeta = PathBuf;
    type AttachmentMeta = PathBuf;
}

pub struct FileSystemStorage {
    pub root: PathBuf,
}

impl FileSystemStorage {}

impl TypedItemStorage<FileSystem> for FileSystemStorage {
    fn build_note(&self, name: &str) -> Note<FileSystem> {
        let path = self.root.join(name);
        Note::new(name, path)
    }

    fn build_collection(&self, name: &str) -> Collection<FileSystem> {
        let path = self.root.join(name);
        Collection::new(name, path)
    }

    fn build_attachment(&self, name: &str) -> Attachment<FileSystem> {
        let path = self.root.join(name);
        Attachment::new(name, path)
    }

    fn save(&self, note: &Note<FileSystem>) -> Result<(), String> {
        println!("Saving FS note: {} = {}", note.name, note.content);
        Ok(())
    }

    fn list_items(&self) -> Result<Vec<Box<dyn AnyItem>>, ReadError> {
        todo!()
    }

    fn save_item(&self, item: &dyn AnyItem) -> Result<(), String> {
        todo!()
    }

    fn save_content(&self, note: &Note<FileSystem>) -> Result<(), WriteError> {
        todo!()
    }
}
