use std::path::PathBuf;

use async_trait::async_trait;
use gtk::gio::prelude::*;
use gtk::glib::DateTime;
use gtk::{gio, glib};

use crate::errors::{ReadError, WriteError};

use super::models::Meta;
use super::storage::StorageBackend;
use super::{
    models::{AnyItem, Attachment, Collection, Note},
    storage::{NoteContent, TypedItemStorage},
};

pub struct Filesystem;

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

    fn filesystem_meta_for_name(&self, name: &str) -> FilesystemMeta {
        let file = self.root.child(name);
        FilesystemMeta {
            file,
            updated_at: DateTime::now_local().unwrap(), // this is safe until the year 9999
        }
    }
}

#[async_trait(?Send)]
    fn build_note(&self, name: &str) -> Note<Filesystem> {
        Note::new(name, self.filesystem_meta_for_name(name))
    }

    fn build_collection(&self, name: &str) -> Collection<Filesystem> {
        Collection::new(name, self.filesystem_meta_for_name(name))
    }

    fn build_attachment(&self, name: &str) -> Attachment<Filesystem> {
        Attachment::new(name, self.filesystem_meta_for_name(name))
impl TypedItemStorage<Filesystem> for FilesystemStorage {
    }

    async fn list_items(&self) -> Result<Vec<Box<dyn AnyItem>>, ReadError> {
        let file_infos = self
            .root
            .enumerate_children_future(
                "standard::*,time::*",
                gio::FileQueryInfoFlags::NONE,
                glib::Priority::DEFAULT,
            )
            .await?;

        let result: Vec<Box<dyn AnyItem>> = file_infos
            .map(|file_info| {
                let file_info = file_info.unwrap();
                let file = self.root.child(file_info.name());

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
