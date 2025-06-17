use std::slice::Iter;
use std::{any::Any, marker::PhantomData};

use gtk::glib::DateTime;

use super::storage::StorageBackend;

pub trait Meta: Send {
    fn updated_at(&self) -> DateTime;
    fn location(&self) -> String;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemKind {
    Note,
    Collection,
    Attachment,
}

pub trait AnyItem: std::fmt::Debug + Any + Send {
    fn kind(&self) -> ItemKind;
    fn as_any(&self) -> &dyn Any;
    fn name(&self) -> String;
    fn updated_at(&self) -> DateTime;
    fn location(&self) -> String;
    fn clone_box(&self) -> Box<dyn AnyItem>;
    fn as_note(&self) -> Option<Box<dyn AnyNote>>;
    fn as_collection(&self) -> Option<Box<dyn AnyCollection>>;
    fn as_attachment(&self) -> Option<Box<dyn AnyAttachment>>;
}

impl PartialEq for &dyn AnyItem {
    fn eq(&self, other: &Self) -> bool {
        self.location() == other.location()
    }
}

pub trait AnyNote: AnyItem {}

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

impl PartialEq for Box<dyn AnyNote> {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl PartialEq for Box<dyn AnyCollection> {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl PartialEq for Box<dyn AnyAttachment> {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

// item models
pub struct Note<S: StorageBackend> {
    pub name: String,
    pub meta: S::NoteMeta,
    _marker: PhantomData<S>,
}

impl<S: StorageBackend + 'static + Send> Note<S> {
    pub fn from_any(note: &dyn AnyNote) -> Option<&Note<S>> {
        note.as_any().downcast_ref::<Note<S>>()
    }

    pub fn new(name: impl Into<String>, meta: S::NoteMeta) -> Self {
        Self {
            name: name.into(),
            meta,
            _marker: PhantomData,
        }
    }
}

impl<S: StorageBackend + 'static + Send> AnyItem for Note<S>
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

    fn updated_at(&self) -> DateTime {
        self.meta.updated_at()
    }

    fn location(&self) -> String {
        self.meta.location()
    }

    fn clone_box(&self) -> Box<dyn AnyItem> {
        self.as_note().unwrap()
    }

    fn as_note(&self) -> Option<Box<dyn AnyNote>> {
        Some(Box::new(Self {
            name: self.name.clone(),
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

impl<S: StorageBackend + 'static + Send> AnyNote for Note<S> {}

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

impl<S: StorageBackend + 'static + Send> Collection<S> {
    #[allow(dead_code)]
    pub fn from_any(collection: &dyn AnyCollection) -> Option<&Collection<S>> {
        collection.as_any().downcast_ref::<Collection<S>>()
    }

    pub fn new(name: impl Into<String>, meta: S::CollectionMeta) -> Self {
        Self {
            name: name.into(),
            meta,
            _marker: PhantomData,
        }
    }
}

impl<S: StorageBackend + 'static + Send> AnyItem for Collection<S>
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

    fn updated_at(&self) -> DateTime {
        self.meta.updated_at()
    }

    fn location(&self) -> String {
        self.meta.location()
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

impl<S: StorageBackend + 'static + Send> AnyCollection for Collection<S> {}

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

impl<S: StorageBackend + 'static> Attachment<S> {
    #[allow(dead_code)]
    pub fn from_any(attachment: &dyn AnyAttachment) -> Option<&Attachment<S>> {
        attachment.as_any().downcast_ref::<Attachment<S>>()
    }

    pub fn new(name: impl Into<String>, meta: S::AttachmentMeta) -> Self {
        Self {
            name: name.into(),
            meta,
            _marker: PhantomData,
        }
    }
}

impl<S: StorageBackend + 'static + Send> AnyItem for Attachment<S>
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

    fn updated_at(&self) -> DateTime {
        self.meta.updated_at()
    }

    fn location(&self) -> String {
        self.meta.location()
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

impl<S: StorageBackend + 'static + Send> AnyAttachment for Attachment<S> {}

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

#[derive(Clone, Debug)]
pub struct CollectionPath {
    collections: Vec<Box<dyn AnyCollection>>,
}

impl CollectionPath {
    pub fn new(collections: Vec<Box<dyn AnyCollection>>) -> Self {
        if collections.len() == 0 {
            panic!("need a root collection")
        }

        CollectionPath { collections }
    }

    pub fn push(&mut self, collection: Box<dyn AnyCollection>) {
        self.collections.push(collection);
    }

    pub fn parent(&self) -> Option<CollectionPath> {
        if self.collections.len() > 1 {
            let (_, rest) = self.collections.split_last().unwrap();
            Some(Self::new(rest.to_vec()))
        } else {
            None
        }
    }

    pub fn iter(&self) -> Iter<Box<dyn AnyCollection>> {
        self.collections.iter()
    }

    pub fn last(&self) -> &Box<dyn AnyCollection> {
        self.collections.last().unwrap()
    }
}

impl PartialEq for CollectionPath {
    fn eq(&self, other: &Self) -> bool {
        if self.collections.len() != other.collections.len() {
            return false;
        }

        self.collections
            .iter()
            .zip(other.collections.iter())
            .all(|(lhs, rhs)| lhs == rhs)
    }
}

impl From<Vec<Box<dyn AnyCollection>>> for CollectionPath {
    fn from(collections: Vec<Box<dyn AnyCollection>>) -> Self {
        Self { collections }
    }
}

impl From<Box<dyn AnyCollection>> for CollectionPath {
    fn from(collection: Box<dyn AnyCollection>) -> Self {
        let mut collections = Vec::new();
        collections.push(collection);

        Self { collections }
    }
}

#[cfg(test)]
mod tests {
    use gtk::glib::DateTime;

    use super::*;

    #[derive(Debug)]
    struct TestCollection {
        name: String,
    }

    impl TestCollection {
        fn new(name: String) -> Self {
            Self { name }
        }
    }

    impl AnyItem for TestCollection {
        fn kind(&self) -> ItemKind {
            ItemKind::Collection
        }

        fn as_any(&self) -> &dyn Any {
            self
        }

        fn name(&self) -> String {
            self.name.clone()
        }

        fn updated_at(&self) -> DateTime {
            DateTime::from_utc(2025, 6, 16, 11, 30, 0.0).unwrap()
        }

        fn location(&self) -> String {
            String::from("/somewhere/over/the/filesystem")
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
            }))
        }

        fn as_attachment(&self) -> Option<Box<dyn AnyAttachment>> {
            None
        }
    }

    impl AnyCollection for TestCollection {}

    impl PartialEq for TestCollection {
        fn eq(&self, other: &Self) -> bool {
            self.name == other.name
        }
    }

    #[test]
    #[should_panic(expected = "need a root collection")]
    fn collection_new_ensures_at_least_one_collection_contained() {
        let _ = CollectionPath::new(vec![]);
    }

    #[test]
    fn collection_new_works() {
        let _ = CollectionPath::new(vec![Box::new(TestCollection::new(String::from("a")))]);
    }

    #[test]
    fn collection_path_parent_returns_parent_path() {
        let path = CollectionPath::new(vec![
            Box::new(TestCollection::new(String::from("a"))),
            Box::new(TestCollection::new(String::from("b"))),
        ]);

        assert!(path.parent().is_some());
        assert!(
            path.parent().unwrap()
                == CollectionPath::new(vec![Box::new(TestCollection::new(String::from("a")))])
        );
    }

    #[test]
    fn collection_path_parent_returns_none_for_root() {
        let path = CollectionPath::new(vec![Box::new(TestCollection::new(String::from("a")))]);

        assert!(path.parent().is_none());
    }
}
