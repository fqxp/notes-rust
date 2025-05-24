use crate::errors::{ReadError, WriteError};

use super::models::{
    AnyAttachment, AnyCollection, AnyItem, AnyNote, Attachment, Collection, Note, StorageBackend,
};

// typed storage
pub trait TypedItemStorage<S: StorageBackend> {
    fn list_items(&self) -> Result<Vec<Box<dyn AnyItem>>, ReadError>;
    fn save_item(&self, item: &dyn AnyItem) -> Result<(), String>;
    fn build_note(&self, name: &str) -> Note<S>;
    fn build_collection(&self, name: &str) -> Collection<S>;
    fn build_attachment(&self, name: &str) -> Attachment<S>;
    fn save(&self, note: &Note<S>) -> Result<(), String>;
    fn save_content(&self, note: &Note<S>) -> Result<(), WriteError>;
}

// type-erased storage
pub trait ItemStorage {
    fn list_items(&self) -> Result<Vec<Box<dyn AnyItem>>, ReadError>;
    fn build_note(&self, name: &str) -> Box<dyn AnyNote>;
    fn build_collection(&self, name: &str) -> Box<dyn AnyCollection>;
    fn build_attachment(&self, name: &str) -> Box<dyn AnyAttachment>;
    fn save(&self, note: &dyn AnyItem) -> Result<(), String>;
    fn save_content(&self, note: &dyn AnyNote) -> Result<(), WriteError>;
}

// type-erased wrapper for type storage
pub(super) struct DynItemStorage<S: StorageBackend> {
    pub inner: Box<dyn TypedItemStorage<S>>,
}

impl<S: StorageBackend + 'static> ItemStorage for DynItemStorage<S> {
    fn list_items(&self) -> Result<Vec<Box<dyn AnyItem>>, ReadError> {
        self.inner.list_items()
    }

    fn build_note(&self, name: &str) -> Box<dyn AnyNote> {
        Box::new(self.inner.build_note(name))
    }

    fn build_collection(&self, name: &str) -> Box<dyn AnyCollection> {
        Box::new(self.inner.build_collection(name))
    }

    fn build_attachment(&self, name: &str) -> Box<dyn AnyAttachment> {
        Box::new(self.inner.build_attachment(name))
    }

    fn save(&self, note: &dyn AnyItem) -> Result<(), String> {
        todo!()
    }

    fn save_content(&self, note: &dyn AnyNote) -> Result<(), WriteError> {
        todo!()
    }
}
