use async_trait::async_trait;

use crate::errors::{ReadError, WriteError};

use super::models::{AnyCollection, AnyItem, AnyNote, Collection, CollectionPath, Meta, Note};

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
    async fn root(&self) -> Result<Collection<S>, ReadError>;
    async fn list_items(&self, path: &CollectionPath) -> Result<Vec<Box<dyn AnyItem>>, ReadError>;
    async fn rename_note(
        &self,
        note: &Note<S>,
        new_name: String,
    ) -> Result<Box<dyn AnyNote>, WriteError>;
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
    async fn root(&self) -> Result<Box<dyn AnyCollection>, ReadError>;
    async fn list_items(&self, path: &CollectionPath) -> Result<Vec<Box<dyn AnyItem>>, ReadError>;
    async fn rename_note(
        &self,
        note: Box<&dyn AnyNote>,
        new_name: String,
    ) -> Result<Box<dyn AnyNote>, WriteError>;
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
    async fn root(&self) -> Result<Box<dyn AnyCollection>, ReadError> {
        let root = self.inner.root().await?;

        Result::Ok(Box::new(root))
    }

    async fn list_items(&self, path: &CollectionPath) -> Result<Vec<Box<dyn AnyItem>>, ReadError> {
        let typed_items: Vec<Box<dyn AnyItem>> = self.inner.list_items(path).await?;

        Ok(typed_items)
    }

    async fn rename_note(
        &self,
        note: Box<&dyn AnyNote>,
        new_name: String,
    ) -> Result<Box<dyn AnyNote>, WriteError> {
        let note = Note::<S>::from_any(*note).unwrap();

        Ok(self.inner.rename_note(note, new_name).await?)
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
