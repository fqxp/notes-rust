use std::path::PathBuf;

use async_trait::async_trait;
use gtk::gio::prelude::*;
use gtk::glib::DateTime;
use gtk::{gio, glib};

use crate::errors::{ReadError, WriteError};

use super::models::{CollectionPath, Meta};
use super::storage::StorageBackend;
use super::{
    models::{AnyItem, Attachment, Collection, Note},
    storage::{NoteContent, TypedItemStorage},
};

#[derive(Debug, Clone)]
pub struct FilesystemMeta {
    file: gio::File,
    updated_at: DateTime,
}

impl Meta for FilesystemMeta {
    fn updated_at(&self) -> DateTime {
        self.updated_at.clone()
    }
}

pub struct Filesystem;

impl StorageBackend for Filesystem {
    type NoteMeta = FilesystemMeta;
    type CollectionMeta = FilesystemMeta;
    type AttachmentMeta = FilesystemMeta;
}

pub struct FilesystemStorage {
    pub root: gio::File,
}

impl FilesystemStorage {
    pub fn new(root: &PathBuf) -> Self {
        Self {
            root: gio::File::for_path(root),
        }
    }

    async fn meta_for_root(&self) -> Result<FilesystemMeta, ReadError> {
        let file = self.root.clone();
        let file_info = file
            .query_info_future(
                "time::*",
                gio::FileQueryInfoFlags::NONE,
                glib::Priority::DEFAULT,
            )
            .await?;

        Result::Ok(FilesystemMeta {
            file,
            updated_at: file_info
                .modification_date_time()
                .expect("valid modification time"),
        })
    }
}

#[async_trait(?Send)]
impl TypedItemStorage<Filesystem> for FilesystemStorage {
    async fn root(&self) -> Result<Collection<Filesystem>, ReadError> {
        let collection = self.meta_for_root().await?;

        Result::Ok(Collection::new("/", collection))
    }

    async fn list_items(&self, path: &CollectionPath) -> Result<Vec<Box<dyn AnyItem>>, ReadError> {
        let collection = path
            .last()
            .as_any()
            .downcast_ref::<Collection<Filesystem>>()
            .unwrap();
        let dir = &collection.meta.file;

        let file_infos = dir
            .enumerate_children_future(
                "standard::*,time::*",
                gio::FileQueryInfoFlags::NONE,
                glib::Priority::DEFAULT,
            )
            .await?;

        let result: Vec<Box<dyn AnyItem>> = file_infos
            .map(|file_info| {
                let file_info = file_info.unwrap();
                let file = dir.child(file_info.name());

                match file_info.file_type() {
                    gio::FileType::Regular => Box::new(Note::<Filesystem>::new(
                        file_info.name().to_str().unwrap(),
                        FilesystemMeta {
                            file,
                            updated_at: file_info
                                .modification_date_time()
                                .expect("modification time should be set"),
                        },
                    )) as Box<dyn AnyItem>,
                    gio::FileType::Directory => Box::new(Collection::<Filesystem>::new(
                        file_info.name().to_str().unwrap(),
                        FilesystemMeta {
                            file,
                            updated_at: file_info
                                .modification_date_time()
                                .expect("modification time should be set"),
                        },
                    )) as Box<dyn AnyItem>,
                    _ => Box::new(Attachment::<Filesystem>::new(
                        file_info.name().to_str().unwrap(),
                        FilesystemMeta {
                            file,
                            updated_at: file_info
                                .modification_date_time()
                                .expect("modification time should be set"),
                        },
                    )) as Box<dyn AnyItem>,
                }
            })
            .collect();

        Result::Ok(result)
    }

    async fn load_content(&self, note: &Note<Filesystem>) -> Result<NoteContent, ReadError> {
        let file = &note.meta.file;
        let (content, etag) = file.load_contents_future().await?;
        let content = String::from_utf8(content.to_vec())?;
        let etag = etag.and_then(|g_string| Some(g_string.to_string()));
        println!("load_content etag={:?}", &etag);

        return Ok(NoteContent { content, etag });
    }

    async fn save_content(
        &self,
        note: &Note<Filesystem>,
        content: &NoteContent,
    ) -> Result<String, WriteError> {
        let (_, etag_after_save) = note
            .meta
            .file
            .replace_contents_future(
                content.content.as_bytes().to_vec(),
                content.etag.as_deref(),
                false,
                gio::FileCreateFlags::NONE,
            )
            .await?;
        println!("save_content etag={:?}", &content.etag);

        Result::Ok(etag_after_save.to_string())
    }
}
