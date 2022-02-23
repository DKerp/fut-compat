use std::path::{Path, PathBuf};
use std::fs::{Metadata, Permissions, FileType};
use std::ffi::OsString;

use futures::stream::Stream;

use async_trait::async_trait;



/// Contains the compatibility objects for the [`tokio`](https://docs.rs/tokio) runtime.
#[cfg(feature = "tokio")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "tokio")))]
mod tokio;
#[cfg(feature = "tokio")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "tokio")))]
pub use self::tokio::*;

/// Contains the compatibility objects for the [`async_std`](https://docs.rs/async-std) runtime.
#[cfg(feature = "async-std")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "async-std")))]
mod async_std;
#[cfg(feature = "async-std")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "async-std")))]
pub use self::async_std::*;



/// An async abstraction over the functions in [`std::fs`].
#[async_trait]
pub trait Filesystem {
    type ReadDir: Stream<Item = std::io::Result<Self::DirEntry>>;
    type DirEntry: DirEntry;

    async fn canonicalize<P: AsRef<Path> + Send>(path: P) -> std::io::Result<PathBuf>;

    async fn copy<S: AsRef<Path> + Send, D: AsRef<Path> + Send>(
        from: S,
        to: D,
    ) -> std::io::Result<u64>;

    async fn create_dir<P: AsRef<Path> + Send>(path: P) -> std::io::Result<()>;

    async fn create_dir_all<P: AsRef<Path> + Send>(path: P) -> std::io::Result<()>;

    async fn hard_link<S: AsRef<Path> + Send, D: AsRef<Path> + Send>(
        from: S,
        to: D,
    ) -> std::io::Result<()>;

    async fn metadata<P: AsRef<Path> + Send>(path: P) -> std::io::Result<Metadata>;

    async fn read<P: AsRef<Path> + Send>(path: P) -> std::io::Result<Vec<u8>>;

    async fn read_dir<P: AsRef<Path> + Send>(path: P) -> std::io::Result<Self::ReadDir>;

    async fn read_link<P: AsRef<Path> + Send>(path: P) -> std::io::Result<PathBuf>;

    async fn read_to_string<P: AsRef<Path> + Send>(path: P) -> std::io::Result<String>;

    async fn remove_dir<P: AsRef<Path> + Send>(path: P) -> std::io::Result<()>;

    async fn remove_dir_all<P: AsRef<Path> + Send>(path: P) -> std::io::Result<()>;

    async fn remove_file<P: AsRef<Path> + Send>(path: P) -> std::io::Result<()>;

    async fn rename<O: AsRef<Path> + Send, N: AsRef<Path> + Send>(
        from: O,
        to: N,
    ) -> std::io::Result<()>;

    async fn set_permissions<P: AsRef<Path> + Send>(
        path: P,
        perm: Permissions,
    ) -> std::io::Result<()>;

    async fn symlink_metadata<P: AsRef<Path> + Send>(path: P) -> std::io::Result<Metadata>;

    async fn write<P: AsRef<Path> + Send, C: AsRef<[u8]> + Send>(
        path: P,
        contents: C
    ) -> std::io::Result<()>;
}



/// An async abstraction over [`std::fs::DirEntry`].
#[async_trait]
pub trait DirEntry {
    fn path(&self) -> PathBuf;

    fn file_name(&self) -> OsString;

    async fn metadata(&self) -> std::io::Result<Metadata>;

    async fn file_type(&self) -> std::io::Result<FileType>;
}



/// An async abstraction over [`std::fs::File`].
#[async_trait]
pub trait File: Sized {
    async fn open<P: AsRef<Path> + Send>(path: P) -> std::io::Result<Self>;

    async fn create<P: AsRef<Path> + Send>(path: P) -> std::io::Result<Self>;

    async fn sync_all(&self) -> std::io::Result<()>;

    async fn sync_data(&self) -> std::io::Result<()>;

    async fn set_len(&self, size: u64) -> std::io::Result<()>;

    async fn set_permissions(&self, perm: Permissions) -> std::io::Result<()>;
}

/// An async abstraction over [`std::fs::OpenOptions`].
#[async_trait]
pub trait OpenOptions: Sized {
    type File: File;

    fn new() -> Self;

    fn read(&mut self, read: bool) -> &mut Self;

    fn write(&mut self, write: bool) -> &mut Self;

    fn append(&mut self, append: bool) -> &mut Self;

    fn truncate(&mut self, truncate: bool) -> &mut Self;

    fn create(&mut self, create: bool) -> &mut Self;

    fn create_new(&mut self, create_new: bool) -> &mut Self;

    async fn open<P: AsRef<Path> + Send>(&self, path: P) -> std::io::Result<Self::File>;
}

/// An async abstraction over [`std::fs::DirBuilder`].
#[async_trait]
pub trait DirBuilder: Sized {
    fn new() -> Self;

    fn recursive(&mut self, recursive: bool) -> &mut Self;

    async fn create<P: AsRef<Path> + Send>(&self, path: P) -> std::io::Result<()>;
}
