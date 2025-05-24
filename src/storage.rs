use std::{any::Any, marker::PhantomData, path::PathBuf, str::FromStr};

use crate::errors::{ReadError, WriteError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemKind {
    Note,
    Collection,
    Attachment,
}

pub trait AnyItem: std::fmt::Debug + Any {
    fn kind(&self) -> ItemKind;
    fn as_any(&self) -> &dyn Any;
    fn name(&self) -> String;
    fn clone_box(&self) -> Box<dyn AnyItem>;
    fn as_note(&self) -> Option<Box<dyn AnyNote>>;
    fn as_collection(&self) -> Option<Box<dyn AnyCollection>>;
    fn as_attachment(&self) -> Option<Box<dyn AnyAttachment>>;
}

pub trait AnyNote: AnyItem {
    fn get_content(&self) -> String;
}

pub trait AnyCollection: AnyItem {}

pub trait AnyAttachment: AnyItem {}

impl Clone for Box<dyn AnyItem> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl Clone for Box<dyn AnyNote> {
    fn clone(&self) -> Self {
        self.as_note().unwrap()
    }
}

impl Clone for Box<dyn AnyCollection> {
    fn clone(&self) -> Self {
        self.as_collection().unwrap()
    }
}

impl Clone for Box<dyn AnyAttachment> {
    fn clone(&self) -> Self {
        self.as_attachment().unwrap()
    }
}

// backend marker trait
pub trait StorageBackend {
    type NoteMeta: std::fmt::Debug + Clone + 'static;
    type CollectionMeta: std::fmt::Debug + Clone + 'static;
    type AttachmentMeta: std::fmt::Debug + Clone + 'static;
}

// item models
pub struct Note<S: StorageBackend> {
    pub name: String,
    pub content: String,
    pub meta: S::NoteMeta,
    _marker: PhantomData<S>,
}

impl<S: StorageBackend> Note<S> {
    pub fn new(name: impl Into<String>, meta: S::NoteMeta) -> Self {
        Self {
            name: name.into(),
            content: String::new(),
            meta,
            _marker: PhantomData,
        }
    }

    pub fn set_content(&mut self, content: impl Into<String>) {
        self.content = content.into();
    }
}

impl<S: StorageBackend + 'static> AnyItem for Note<S>
where
    S::NoteMeta: Clone,
{
    fn kind(&self) -> ItemKind {
        ItemKind::Note
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn clone_box(&self) -> Box<dyn AnyItem> {
        self.as_note().unwrap()
    }

    fn as_note(&self) -> Option<Box<dyn AnyNote>> {
        Some(Box::new(Self {
            name: self.name.clone(),
            content: self.content.clone(),
            meta: self.meta.clone(),
            _marker: PhantomData,
        }))
    }

    fn as_collection(&self) -> Option<Box<dyn AnyCollection>> {
        None
    }

    fn as_attachment(&self) -> Option<Box<dyn AnyAttachment>> {
        None
    }
}

impl<S: StorageBackend + 'static> AnyNote for Note<S> {
    fn get_content(&self) -> String {
        self.content.clone()
    }
}

impl<S: StorageBackend> std::fmt::Debug for Note<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Note<{}>({})", std::any::type_name::<S>(), self.name)
    }
}

pub struct Collection<S: StorageBackend> {
    pub name: String,
    pub meta: S::CollectionMeta,
    _marker: PhantomData<S>,
}

impl<S: StorageBackend> Collection<S> {
    pub fn new(name: impl Into<String>, meta: S::CollectionMeta) -> Self {
        Self {
            name: name.into(),
            meta,
            _marker: PhantomData,
        }
    }
}

impl<S: StorageBackend + 'static> AnyItem for Collection<S>
where
    S::CollectionMeta: Clone,
{
    fn kind(&self) -> ItemKind {
        ItemKind::Collection
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn clone_box(&self) -> Box<dyn AnyItem> {
        self.as_collection().unwrap()
    }

    fn as_note(&self) -> Option<Box<dyn AnyNote>> {
        None
    }

    fn as_collection(&self) -> Option<Box<dyn AnyCollection>> {
        Some(Box::new(Self {
            name: self.name.clone(),
            meta: self.meta.clone(),
            _marker: PhantomData,
        }))
    }

    fn as_attachment(&self) -> Option<Box<dyn AnyAttachment>> {
        None
    }
}

impl<S: StorageBackend + 'static> AnyCollection for Collection<S> {}

impl<S: StorageBackend> std::fmt::Debug for Collection<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Collection<{}>({})",
            std::any::type_name::<S>(),
            self.name
        )
    }
}

pub struct Attachment<S: StorageBackend> {
    pub name: String,
    pub meta: S::AttachmentMeta,
    _marker: PhantomData<S>,
}

impl<S: StorageBackend> Attachment<S> {
    pub fn new(name: impl Into<String>, meta: S::AttachmentMeta) -> Self {
        Self {
            name: name.into(),
            meta,
            _marker: PhantomData,
        }
    }
}

impl<S: StorageBackend + 'static> AnyItem for Attachment<S>
where
    S::AttachmentMeta: Clone,
{
    fn kind(&self) -> ItemKind {
        ItemKind::Attachment
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn clone_box(&self) -> Box<dyn AnyItem> {
        self.as_attachment().unwrap()
    }

    fn as_note(&self) -> Option<Box<dyn AnyNote>> {
        None
    }

    fn as_collection(&self) -> Option<Box<dyn AnyCollection>> {
        None
    }

    fn as_attachment(&self) -> Option<Box<dyn AnyAttachment>> {
        Some(Box::new(Self {
            name: self.name.clone(),
            meta: self.meta.clone(),
            _marker: PhantomData,
        }))
    }
}

impl<S: StorageBackend + 'static> AnyAttachment for Attachment<S> {}

impl<S: StorageBackend> std::fmt::Debug for Attachment<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Attachment<{}>({})",
            std::any::type_name::<S>(),
            self.name
        )
    }
}

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
pub struct DynItemStorage<S: StorageBackend> {
    inner: Box<dyn TypedItemStorage<S>>,
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

pub fn build_storage_from_url(url: &str) -> Box<dyn ItemStorage> {
    if url.starts_with("file://") {
        let root = PathBuf::from_str(url.strip_prefix("file://").unwrap()).unwrap();
        let fs_storage = Box::new(FileSystemStorage { root });
        Box::new(DynItemStorage { inner: fs_storage })
    } else {
        panic!("unknown storage URL {}", url);
    }
}
