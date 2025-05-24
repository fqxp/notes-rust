use std::{any::Any, marker::PhantomData};

// backend marker trait
pub trait StorageBackend {
    type NoteMeta: std::fmt::Debug + Clone + 'static;
    type CollectionMeta: std::fmt::Debug + Clone + 'static;
    type AttachmentMeta: std::fmt::Debug + Clone + 'static;
}

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
