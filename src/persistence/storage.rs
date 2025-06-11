use async_trait::async_trait;

use crate::errors::{ReadError, WriteError};

use super::models::{
    AnyAttachment, AnyCollection, AnyItem, AnyNote, Attachment, Collection, Note, StorageBackend,
};

pub struct NoteContent {
    pub content: String,
    pub etag: Option<String>,
}

// backend marker trait
pub trait StorageBackend {
    type NoteMeta: Meta + std::fmt::Debug + Clone + 'static;
    type CollectionMeta: Meta + std::fmt::Debug + Clone + 'static;
    type AttachmentMeta: Meta + std::fmt::Debug + Clone + 'static;
}

// typed storage
#[async_trait(?Send)]
pub trait TypedItemStorage<S: StorageBackend>: Send + Sync {
    fn build_note(&self, name: &str) -> Note<S>;
    fn build_collection(&self, name: &str) -> Collection<S>;
    fn build_attachment(&self, name: &str) -> Attachment<S>;
    async fn list_items(&self) -> Result<Vec<Box<dyn AnyItem>>, ReadError>;
    async fn load_content(&self, note: &Note<S>) -> Result<NoteContent, ReadError>;
    async fn save_content(
        &self,
        note: &Note<S>,
        content: &NoteContent,
    ) -> Result<String, WriteError>;
}

// type-erased storage
#[async_trait(?Send)]
pub trait ItemStorage {
    fn build_note(&self, name: &str) -> Box<dyn AnyNote>;
    fn build_collection(&self, name: &str) -> Box<dyn AnyCollection>;
    fn build_attachment(&self, name: &str) -> Box<dyn AnyAttachment>;
    async fn list_items(&self) -> Result<Vec<Box<dyn AnyItem>>, ReadError>;
    async fn load_content(&self, note: &dyn AnyNote) -> Result<NoteContent, ReadError>;
    async fn save_content(
        &self,
        note: &dyn AnyNote,
        content: &NoteContent,
    ) -> Result<String, WriteError>;
}

// type-erased wrapper for typed storage
pub(super) struct DynItemStorage<S: StorageBackend> {
    pub inner: Box<dyn TypedItemStorage<S> + Send + Sync>,
}

#[async_trait(?Send)]
impl<S: StorageBackend + 'static + Send> ItemStorage for DynItemStorage<S> {
    fn build_note(&self, name: &str) -> Box<dyn AnyNote> {
        Box::new(self.inner.build_note(name))
    }

    fn build_collection(&self, name: &str) -> Box<dyn AnyCollection> {
        Box::new(self.inner.build_collection(name))
    }

    fn build_attachment(&self, name: &str) -> Box<dyn AnyAttachment> {
        Box::new(self.inner.build_attachment(name))
    }

    async fn list_items(&self) -> Result<Vec<Box<dyn AnyItem>>, ReadError> {
        let typed_items: Vec<Box<dyn AnyItem>> = self.inner.list_items().await?;

        Ok(typed_items)
    }

    async fn load_content(&self, note: &dyn AnyNote) -> Result<NoteContent, ReadError> {
        let note = Note::<S>::from_any(note).unwrap();
        let content: NoteContent = self.inner.load_content(note).await?;

        Ok(content)
    }

    async fn save_content(
        &self,
        note: &dyn AnyNote,
        content: &NoteContent,
    ) -> Result<String, WriteError> {
        let note = Note::<S>::from_any(note).unwrap();
        let etag = self.inner.save_content(note, content).await?;

        Ok(etag)
    }
}
