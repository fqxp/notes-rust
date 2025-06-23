use std::path::PathBuf;

use async_trait::async_trait;
use gtk::gio::prelude::*;
use gtk::{gio, glib};

use crate::errors::Error;

use super::models::{AnyNote, CollectionPath, Meta};
use super::storage::StorageBackend;
use super::{
    models::{AnyItem, Attachment, Collection, Note},
    storage::{NoteContent, TypedItemStorage},
};

#[derive(Debug, Clone)]
pub struct FilesystemMeta {}

impl Meta for FilesystemMeta {}

#[derive(Clone)]
pub struct Filesystem;

impl StorageBackend for Filesystem {
    type NoteMeta = FilesystemMeta;
    type CollectionMeta = FilesystemMeta;
    type AttachmentMeta = FilesystemMeta;
}

pub struct FilesystemStorage {
    pub root: Collection<Filesystem>,
}

impl FilesystemStorage {
    pub async fn from_uri(root_uri: &str) -> Result<Self, Error> {
        let file = gio::File::for_uri(root_uri);
        if !file.query_exists(gio::Cancellable::NONE) {
            return Err(Error::DoesNotExist {
                uri: String::from(root_uri),
            });
        }

        let file_info = file
            .query_info_future(
                "time::*",
                gio::FileQueryInfoFlags::NONE,
                glib::Priority::DEFAULT,
            )
            .await?;
        let root = Collection::new(
            FilesystemMeta {},
            file.basename()
                .expect("valid root path")
                .to_string_lossy()
                .to_string(),
            file_info
                .modification_date_time()
                .expect("valid modification datetime"),
            root_uri.to_string(),
        );

        Ok(Self { root })
    }

    fn path_from_uri(&self, uri: String) -> Option<PathBuf> {
        if !uri.starts_with("file://") {
            return None;
        }

        Some(PathBuf::from(uri[4..].to_string()))
    }
}

#[async_trait(?Send)]
impl TypedItemStorage<Filesystem> for FilesystemStorage {
    fn root(&self) -> Box<Collection<Filesystem>> {
        Box::new(self.root.clone())
    }

    async fn list_items(&self, path: &CollectionPath) -> Result<Vec<Box<dyn AnyItem>>, Error> {
        let collection = path
            .last()
            .as_any()
            .downcast_ref::<Collection<Filesystem>>()
            .unwrap();
        let dir = gio::File::for_uri(&collection.location());

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
                        FilesystemMeta {},
                        file_info.name().to_string_lossy().to_string(),
                        file_info
                            .modification_date_time()
                            .expect("modification time should be set"),
                        file.uri().to_string(),
                    )) as Box<dyn AnyItem>,
                    gio::FileType::Directory => Box::new(Collection::<Filesystem>::new(
                        FilesystemMeta {},
                        file_info.name().to_string_lossy().to_string(),
                        file_info
                            .modification_date_time()
                            .expect("modification time should be set"),
                        file.uri().to_string(),
                    )) as Box<dyn AnyItem>,
                    _ => Box::new(Attachment::<Filesystem>::new(
                        FilesystemMeta {},
                        file_info.name().to_string_lossy().to_string(),
                        file_info
                            .modification_date_time()
                            .expect("modification time should be set"),
                        file.uri().to_string(),
                    )) as Box<dyn AnyItem>,
                }
            })
            .collect();

        Result::Ok(result)
    }

    async fn rename_note(
        &self,
        note: &Note<Filesystem>,
        new_name: &str,
    ) -> Result<Box<dyn AnyNote>, Error> {
        let src_file = gio::File::for_uri(&note.location());

        if let Some(dest_file) = src_file.parent().and_then(|f| Some(f.child(new_name))) {
            let (result, _) = src_file.move_future(
                &dest_file,
                gio::FileCopyFlags::NONE,
                glib::Priority::DEFAULT,
            );
            result.await?;

            let file_info = dest_file
                .query_info_future(
                    "time::*",
                    gio::FileQueryInfoFlags::NONE,
                    glib::Priority::DEFAULT,
                )
                .await?;

            Ok(Box::new(Note::<Filesystem>::new(
                FilesystemMeta {},
                String::from(new_name),
                file_info
                    .modification_date_time()
                    .expect("valid modification time"),
                format!("file://{:?}", dest_file.path().expect("valid path")),
            )))
        } else {
            Err(Error::OtherError("error moving file".to_string()))
        }
    }

    async fn load_content(&self, note: &Note<Filesystem>) -> Result<NoteContent, Error> {
        let file = gio::File::for_uri(&note.location());
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
    ) -> Result<String, Error> {
        let file = gio::File::for_uri(&note.location());
        let (_, etag_after_save) = file
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
