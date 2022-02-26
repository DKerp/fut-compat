use super::*;
use crate::io::TokioCompat;

use ::tokio::fs;

use tokio_stream::wrappers::ReadDirStream;



/// [`tokio`](https://docs.rs/tokio)'s abstraction of a [`Filesystem`].
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TokioFs {}


#[async_trait]
impl Filesystem for TokioFs {
    type ReadDir = ReadDirStream;
    type DirEntry = fs::DirEntry;

    async fn canonicalize<P: AsRef<Path> + Send>(path: P) -> std::io::Result<PathBuf> {
        fs::canonicalize(path).await
    }

    async fn copy<S: AsRef<Path> + Send, D: AsRef<Path> + Send>(
        from: S,
        to: D,
    ) -> std::io::Result<u64> {
        fs::copy(from, to).await
    }

    async fn create_dir<P: AsRef<Path> + Send>(path: P) -> std::io::Result<()> {
        fs::create_dir(path).await
    }

    async fn create_dir_all<P: AsRef<Path> + Send>(path: P) -> std::io::Result<()> {
        fs::create_dir_all(path).await
    }

    async fn hard_link<S: AsRef<Path> + Send, D: AsRef<Path> + Send>(
        from: S,
        to: D,
    ) -> std::io::Result<()> {
        fs::hard_link(from, to).await
    }

    async fn metadata<P: AsRef<Path> + Send>(path: P) -> std::io::Result<Metadata> {
        fs::metadata(path).await
    }

    async fn read<P: AsRef<Path> + Send>(path: P) -> std::io::Result<Vec<u8>> {
        fs::read(path).await
    }

    async fn read_dir<P: AsRef<Path> + Send>(path: P) -> std::io::Result<Self::ReadDir> {
        fs::read_dir(path).await.map(|read_dir| ReadDirStream::new(read_dir))
    }

    async fn read_link<P: AsRef<Path> + Send>(path: P) -> std::io::Result<PathBuf> {
        fs::read_link(path).await
    }

    async fn read_to_string<P: AsRef<Path> + Send>(path: P) -> std::io::Result<String> {
        fs::read_to_string(path).await
    }

    async fn remove_dir<P: AsRef<Path> + Send>(path: P) -> std::io::Result<()> {
        fs::remove_dir(path).await
    }

    async fn remove_dir_all<P: AsRef<Path> + Send>(path: P) -> std::io::Result<()> {
        fs::remove_dir(path).await
    }

    async fn remove_file<P: AsRef<Path> + Send>(path: P) -> std::io::Result<()> {
        fs::remove_file(path).await
    }

    async fn rename<O: AsRef<Path> + Send, N: AsRef<Path> + Send>(
        from: O,
        to: N,
    ) -> std::io::Result<()> {
        fs::rename(from, to).await
    }

    async fn set_permissions<P: AsRef<Path> + Send>(
        path: P,
        perm: Permissions,
    ) -> std::io::Result<()> {
        fs::set_permissions(path, perm).await
    }

    async fn symlink_metadata<P: AsRef<Path> + Send>(path: P) -> std::io::Result<Metadata> {
        fs::symlink_metadata(path).await
    }

    async fn write<P: AsRef<Path> + Send, C: AsRef<[u8]> + Send>(
        path: P,
        contents: C
    ) -> std::io::Result<()> {
        fs::write(path, contents).await
    }
}

#[async_trait]
impl DirEntry for fs::DirEntry {
    fn path(&self) -> PathBuf {
        self.path()
    }

    fn file_name(&self) -> OsString {
        self.file_name()
    }

    async fn metadata(&self) -> std::io::Result<Metadata> {
        self.metadata().await
    }

    async fn file_type(&self) -> std::io::Result<FileType> {
        self.file_type().await
    }
}

#[async_trait]
impl File for fs::File {
    async fn open<P: AsRef<Path> + Send>(path: P) -> std::io::Result<Self> {
        Self::open(path).await
    }

    async fn create<P: AsRef<Path> + Send>(path: P) -> std::io::Result<Self> {
        Self::create(path).await
    }

    async fn sync_all(&self) -> std::io::Result<()> {
        self.sync_all().await
    }

    async fn sync_data(&self) -> std::io::Result<()> {
        self.sync_data().await
    }

    async fn set_len(&self, size: u64) -> std::io::Result<()> {
        self.set_len(size).await
    }

    async fn metadata(&self) -> std::io::Result<Metadata> {
        self.metadata().await
    }

    async fn set_permissions(&self, perm: Permissions) -> std::io::Result<()> {
        self.set_permissions(perm).await
    }
}

#[async_trait]
impl File for TokioCompat<fs::File> {
    async fn open<P: AsRef<Path> + Send>(path: P) -> std::io::Result<Self> {
        fs::File::open(path).await.map(|inner| Self::new(inner))
    }

    async fn create<P: AsRef<Path> + Send>(path: P) -> std::io::Result<Self> {
        fs::File::create(path).await.map(|inner| Self::new(inner))
    }

    async fn sync_all(&self) -> std::io::Result<()> {
        self.get_ref().sync_all().await
    }

    async fn sync_data(&self) -> std::io::Result<()> {
        self.get_ref().sync_data().await
    }

    async fn set_len(&self, size: u64) -> std::io::Result<()> {
        self.get_ref().set_len(size).await
    }

    async fn metadata(&self) -> std::io::Result<Metadata> {
        self.get_ref().metadata().await
    }

    async fn set_permissions(&self, perm: Permissions) -> std::io::Result<()> {
        self.get_ref().set_permissions(perm).await
    }
}

#[async_trait]
impl OpenOptions for fs::OpenOptions {
    type File = TokioCompat<fs::File>;

    fn new() -> Self {
        Self::new()
    }

    fn read(&mut self, read: bool) -> &mut Self {
        self.read(read)
    }

    fn write(&mut self, write: bool) -> &mut Self {
        self.write(write)
    }

    fn append(&mut self, append: bool) -> &mut Self {
        self.append(append)
    }

    fn truncate(&mut self, truncate: bool) -> &mut Self {
        self.truncate(truncate)
    }

    fn create(&mut self, create: bool) -> &mut Self {
        self.create(create)
    }

    fn create_new(&mut self, create_new: bool) -> &mut Self {
        self.create_new(create_new)
    }

    async fn open<P: AsRef<Path> + Send>(&self, path: P) -> std::io::Result<Self::File> {
        self.open(path).await.map(|inner| Self::File::new(inner))
    }
}

#[async_trait]
impl DirBuilder for fs::DirBuilder {
    fn new() -> Self {
        Self::new()
    }

    fn recursive(&mut self, recursive: bool) -> &mut Self {
        self.recursive(recursive)
    }

    async fn create<P: AsRef<Path> + Send>(&self, path: P) -> std::io::Result<()> {
        self.create(path).await
    }
}
