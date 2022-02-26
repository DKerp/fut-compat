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

    /// Returns the canonical form of a path.
    ///
    /// The returned path is in absolute form with all intermediate components normalized and symbolic
    /// links resolved.
    ///
    /// This function is an async version of [`std::fs::canonicalize`].
    ///
    /// [`std::fs::canonicalize`]: https://doc.rust-lang.org/std/fs/fn.canonicalize.html
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `path` does not point to an existing file or directory.
    /// * A non-final component in `path` is not a directory.
    /// * Some other I/O error occurred.
    ///
    /// # Examples
    ///
    /// Using the [`tokio`](https://docs.rs/tokio) runtime:
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> std::io::Result<()> {
    /// #
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::TokioFs;
    ///
    /// let path = TokioFs::canonicalize(".").await?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`async_std`](https://docs.rs/async-std) runtime:
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::AsyncStdFs;
    ///
    /// let path = AsyncStdFs::canonicalize(".").await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    async fn canonicalize<P: AsRef<Path> + Send>(path: P) -> std::io::Result<PathBuf>;

    /// Copies the contents and permissions of a file to a new location.
    ///
    /// On success, the total number of bytes copied is returned and equals the length of the `to` file
    /// after this operation.
    ///
    /// The old contents of `to` will be overwritten. If `from` and `to` both point to the same file,
    /// then the file will likely get truncated as a result of this operation.
    ///
    /// If you're working with open [`File`]s and want to copy contents through those types, use the
    /// [`io::copy`] function.
    ///
    /// This function is an async version of [`std::fs::copy`].
    ///
    /// [`File`]: struct.File.html
    /// [`io::copy`]: ../io/fn.copy.html
    /// [`std::fs::copy`]: https://doc.rust-lang.org/std/fs/fn.copy.html
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `from` does not point to an existing file.
    /// * The current process lacks permissions to read `from` or write `to`.
    /// * Some other I/O error occurred.
    ///
    /// # Examples
    ///
    /// Using the [`tokio`](https://docs.rs/tokio) runtime:
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> std::io::Result<()> {
    /// #
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::TokioFs;
    ///
    /// let num_bytes = TokioFs::copy("a.txt", "b.txt").await?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`async_std`](https://docs.rs/async-std) runtime:
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::AsyncStdFs;
    ///
    /// let num_bytes = AsyncStdFs::copy("a.txt", "b.txt").await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    async fn copy<S: AsRef<Path> + Send, D: AsRef<Path> + Send>(
        from: S,
        to: D,
    ) -> std::io::Result<u64>;

    /// Creates a new directory.
    ///
    /// Note that this function will only create the final directory in `path`. If you want to create
    /// all of its missing parent directories too, use the [`create_dir_all`] function instead.
    ///
    /// This function is an async version of [`std::fs::create_dir`].
    ///
    /// [`create_dir_all`]: fn.create_dir_all.html
    /// [`std::fs::create_dir`]: https://doc.rust-lang.org/std/fs/fn.create_dir.html
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `path` already points to an existing file or directory.
    /// * A parent directory in `path` does not exist.
    /// * The current process lacks permissions to create the directory.
    /// * Some other I/O error occurred.
    ///
    /// # Examples
    ///
    /// Using the [`tokio`](https://docs.rs/tokio) runtime:
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> std::io::Result<()> {
    /// #
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::TokioFs;
    ///
    /// TokioFs::create_dir("./some/directory").await?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`async_std`](https://docs.rs/async-std) runtime:
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::AsyncStdFs;
    ///
    /// AsyncStdFs::create_dir("./some/directory").await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    async fn create_dir<P: AsRef<Path> + Send>(path: P) -> std::io::Result<()>;

    /// Creates a new directory and all of its parents if they are missing.
    ///
    /// This function is an async version of [`std::fs::create_dir_all`].
    ///
    /// [`std::fs::create_dir_all`]: https://doc.rust-lang.org/std/fs/fn.create_dir_all.html
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `path` already points to an existing file or directory.
    /// * The current process lacks permissions to create the directory or its missing parents.
    /// * Some other I/O error occurred.
    ///
    /// # Examples
    ///
    /// Using the [`tokio`](https://docs.rs/tokio) runtime:
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> std::io::Result<()> {
    /// #
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::TokioFs;
    ///
    /// TokioFs::create_dir_all("./some/directory").await?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`async_std`](https://docs.rs/async-std) runtime:
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::AsyncStdFs;
    ///
    /// AsyncStdFs::create_dir_all("./some/directory").await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    async fn create_dir_all<P: AsRef<Path> + Send>(path: P) -> std::io::Result<()>;

    /// Creates a hard link on the filesystem.
    ///
    /// The `dst` path will be a link pointing to the `src` path. Note that operating systems often
    /// require these two paths to be located on the same filesystem.
    ///
    /// This function is an async version of [`std::fs::hard_link`].
    ///
    /// [`std::fs::hard_link`]: https://doc.rust-lang.org/std/fs/fn.hard_link.html
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `src` does not point to an existing file.
    /// * Some other I/O error occurred.
    ///
    /// # Examples
    ///
    /// Using the [`tokio`](https://docs.rs/tokio) runtime:
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> std::io::Result<()> {
    /// #
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::TokioFs;
    ///
    /// TokioFs::hard_link("a.txt", "b.txt").await?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`async_std`](https://docs.rs/async-std) runtime:
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::AsyncStdFs;
    ///
    /// AsyncStdFs::hard_link("a.txt", "b.txt").await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    async fn hard_link<S: AsRef<Path> + Send, D: AsRef<Path> + Send>(
        from: S,
        to: D,
    ) -> std::io::Result<()>;

    /// Reads metadata for a path.
    ///
    /// This function will traverse symbolic links to read metadata for the target file or directory.
    /// If you want to read metadata without following symbolic links, use [`symlink_metadata`]
    /// instead.
    ///
    /// This function is an async version of [`std::fs::metadata`].
    ///
    /// [`symlink_metadata`]: fn.symlink_metadata.html
    /// [`std::fs::metadata`]: https://doc.rust-lang.org/std/fs/fn.metadata.html
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `path` does not point to an existing file or directory.
    /// * The current process lacks permissions to read metadata for the path.
    /// * Some other I/O error occurred.
    ///
    /// # Examples
    ///
    /// Using the [`tokio`](https://docs.rs/tokio) runtime:
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> std::io::Result<()> {
    /// #
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::TokioFs;
    ///
    /// let meta = TokioFs::metadata("a.txt").await?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`async_std`](https://docs.rs/async-std) runtime:
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::AsyncStdFs;
    ///
    /// let meta = AsyncStdFs::metadata("a.txt").await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    async fn metadata<P: AsRef<Path> + Send>(path: P) -> std::io::Result<Metadata>;

    /// Reads the entire contents of a file as raw bytes.
    ///
    /// This is a convenience function for reading entire files. It pre-allocates a buffer based on the
    /// file size when available, so it is typically faster than manually opening a file and reading
    /// from it.
    ///
    /// If you want to read the contents as a string, use [`read_to_string`] instead.
    ///
    /// This function is an async version of [`std::fs::read`].
    ///
    /// [`read_to_string`]: fn.read_to_string.html
    /// [`std::fs::read`]: https://doc.rust-lang.org/std/fs/fn.read.html
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `path` does not point to an existing file.
    /// * The current process lacks permissions to read the file.
    /// * Some other I/O error occurred.
    ///
    /// # Examples
    ///
    /// Using the [`tokio`](https://docs.rs/tokio) runtime:
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> std::io::Result<()> {
    /// #
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::TokioFs;
    ///
    /// let contents = TokioFs::read("a.txt").await?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`async_std`](https://docs.rs/async-std) runtime:
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::AsyncStdFs;
    ///
    /// let contents = AsyncStdFs::read("a.txt").await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    async fn read<P: AsRef<Path> + Send>(path: P) -> std::io::Result<Vec<u8>>;

    /// Returns a stream of entries in a directory.
    ///
    /// The stream yields items of type [`io::Result`]`<`[`DirEntry`]`>`. Note that I/O errors can
    /// occur while reading from the stream.
    ///
    /// This function is an async version of [`std::fs::read_dir`].
    ///
    /// [`io::Result`]: ../io/type.Result.html
    /// [`DirEntry`]: struct.DirEntry.html
    /// [`std::fs::read_dir`]: https://doc.rust-lang.org/std/fs/fn.read_dir.html
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `path` does not point to an existing directory.
    /// * The current process lacks permissions to read the contents of the directory.
    /// * Some other I/O error occurred.
    ///
    /// # Examples
    ///
    /// Using the [`tokio`](https://docs.rs/tokio) runtime:
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> std::io::Result<()> {
    /// #
    /// use futures::stream::StreamExt;
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::TokioFs;
    ///
    /// let mut entries = TokioFs::read_dir(".").await?;
    ///
    /// while let Some(res) = entries.next().await {
    ///     let entry = res?;
    ///     println!("{}", entry.file_name().to_string_lossy());
    /// }
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`async_std`](https://docs.rs/async-std) runtime:
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use futures::stream::StreamExt;
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::AsyncStdFs;
    ///
    /// let mut entries = AsyncStdFs::read_dir(".").await?;
    ///
    /// while let Some(res) = entries.next().await {
    ///     let entry = res?;
    ///     println!("{}", entry.file_name().to_string_lossy());
    /// }
    /// #
    /// # Ok(()) }) }
    /// ```
    async fn read_dir<P: AsRef<Path> + Send>(path: P) -> std::io::Result<Self::ReadDir>;

    /// Reads a symbolic link and returns the path it points to.
    ///
    /// This function is an async version of [`std::fs::read_link`].
    ///
    /// [`std::fs::read_link`]: https://doc.rust-lang.org/std/fs/fn.read_link.html
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `path` does not point to an existing link.
    /// * Some other I/O error occurred.
    ///
    /// # Examples
    ///
    /// Using the [`tokio`](https://docs.rs/tokio) runtime:
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> std::io::Result<()> {
    /// #
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::TokioFs;
    ///
    /// let path = TokioFs::read_link("a.txt").await?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`async_std`](https://docs.rs/async-std) runtime:
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::AsyncStdFs;
    ///
    /// let path = AsyncStdFs::read_link("a.txt").await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    async fn read_link<P: AsRef<Path> + Send>(path: P) -> std::io::Result<PathBuf>;

    /// Reads the entire contents of a file as a string.
    ///
    /// This is a convenience function for reading entire files. It pre-allocates a string based on the
    /// file size when available, so it is typically faster than manually opening a file and reading
    /// from it.
    ///
    /// If you want to read the contents as raw bytes, use [`read`] instead.
    ///
    /// This function is an async version of [`std::fs::read_to_string`].
    ///
    /// [`read`]: #tymethod.read
    /// [`std::fs::read_to_string`]: https://doc.rust-lang.org/std/fs/fn.read_to_string.html
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `path` does not point to an existing file.
    /// * The current process lacks permissions to read the file.
    /// * The contents of the file cannot be read as a UTF-8 string.
    /// * Some other I/O error occurred.
    ///
    /// # Examples
    ///
    /// Using the [`tokio`](https://docs.rs/tokio) runtime:
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> std::io::Result<()> {
    /// #
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::TokioFs;
    ///
    /// let contents = TokioFs::read_to_string("a.txt").await?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`async_std`](https://docs.rs/async-std) runtime:
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::AsyncStdFs;
    ///
    /// let contents = AsyncStdFs::read_to_string("a.txt").await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    async fn read_to_string<P: AsRef<Path> + Send>(path: P) -> std::io::Result<String>;

    /// Removes an empty directory.
    ///
    /// This function is an async version of [`std::fs::remove_dir`].
    ///
    /// [`std::fs::remove_dir`]: https://doc.rust-lang.org/std/fs/fn.remove_dir.html
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `path` is not an existing and empty directory.
    /// * The current process lacks permissions to remove the directory.
    /// * Some other I/O error occurred.
    ///
    /// # Examples
    ///
    /// Using the [`tokio`](https://docs.rs/tokio) runtime:
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> std::io::Result<()> {
    /// #
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::TokioFs;
    ///
    /// TokioFs::remove_dir("./some/directory").await?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`async_std`](https://docs.rs/async-std) runtime:
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::AsyncStdFs;
    ///
    /// AsyncStdFs::remove_dir("./some/directory").await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    async fn remove_dir<P: AsRef<Path> + Send>(path: P) -> std::io::Result<()>;

    /// Removes a directory and all of its contents.
    ///
    /// This function is an async version of [`std::fs::remove_dir_all`].
    ///
    /// [`std::fs::remove_dir_all`]: https://doc.rust-lang.org/std/fs/fn.remove_dir_all.html
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `path` is not an existing and empty directory.
    /// * The current process lacks permissions to remove the directory.
    /// * Some other I/O error occurred.
    ///
    /// # Examples
    ///
    /// Using the [`tokio`](https://docs.rs/tokio) runtime:
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> std::io::Result<()> {
    /// #
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::TokioFs;
    ///
    /// TokioFs::remove_dir_all("./some/directory").await?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`async_std`](https://docs.rs/async-std) runtime:
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::AsyncStdFs;
    ///
    /// AsyncStdFs::remove_dir_all("./some/directory").await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    async fn remove_dir_all<P: AsRef<Path> + Send>(path: P) -> std::io::Result<()>;

    /// Removes a file.
    ///
    /// This function is an async version of [`std::fs::remove_file`].
    ///
    /// [`std::fs::remove_file`]: https://doc.rust-lang.org/std/fs/fn.remove_file.html
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `path` does not point to an existing file.
    /// * The current process lacks permissions to remove the file.
    /// * Some other I/O error occurred.
    ///
    /// # Examples
    ///
    /// Using the [`tokio`](https://docs.rs/tokio) runtime:
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> std::io::Result<()> {
    /// #
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::TokioFs;
    ///
    /// TokioFs::remove_file("a.txt").await?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`async_std`](https://docs.rs/async-std) runtime:
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::AsyncStdFs;
    ///
    /// AsyncStdFs::remove_file("a.txt").await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    async fn remove_file<P: AsRef<Path> + Send>(path: P) -> std::io::Result<()>;

    /// Renames a file or directory to a new location.
    ///
    /// If a file or directory already exists at the target location, it will be overwritten by this
    /// operation.
    ///
    /// This function is an async version of [`std::fs::rename`].
    ///
    /// [`std::fs::rename`]: https://doc.rust-lang.org/std/fs/fn.rename.html
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `from` does not point to an existing file or directory.
    /// * `from` and `to` are on different filesystems.
    /// * The current process lacks permissions to do the rename operation.
    /// * Some other I/O error occurred.
    ///
    /// # Examples
    ///
    /// Using the [`tokio`](https://docs.rs/tokio) runtime:
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> std::io::Result<()> {
    /// #
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::TokioFs;
    ///
    /// TokioFs::rename("a.txt", "b.txt").await?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`async_std`](https://docs.rs/async-std) runtime:
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::AsyncStdFs;
    ///
    /// AsyncStdFs::rename("a.txt", "b.txt").await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    async fn rename<O: AsRef<Path> + Send, N: AsRef<Path> + Send>(
        from: O,
        to: N,
    ) -> std::io::Result<()>;

    /// Changes the permissions of a file or directory.
    ///
    /// This function is an async version of [`std::fs::set_permissions`].
    ///
    /// [`std::fs::set_permissions`]: https://doc.rust-lang.org/std/fs/fn.set_permissions.html
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `path` does not point to an existing file or directory.
    /// * The current process lacks permissions to change attributes on the file or directory.
    /// * Some other I/O error occurred.
    ///
    /// # Examples
    ///
    /// Using the [`tokio`](https://docs.rs/tokio) runtime:
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> std::io::Result<()> {
    /// #
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::TokioFs;
    ///
    /// let mut perm = TokioFs::metadata("a.txt").await?.permissions();
    /// perm.set_readonly(true);
    /// TokioFs::set_permissions("a.txt", perm).await?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`async_std`](https://docs.rs/async-std) runtime:
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::AsyncStdFs;
    ///
    /// let mut perm = AsyncStdFs::metadata("a.txt").await?.permissions();
    /// perm.set_readonly(true);
    /// AsyncStdFs::set_permissions("a.txt", perm).await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    async fn set_permissions<P: AsRef<Path> + Send>(
        path: P,
        perm: Permissions,
    ) -> std::io::Result<()>;

    /// Reads metadata for a path without following symbolic links.
    ///
    /// If you want to follow symbolic links before reading metadata of the target file or directory,
    /// use [`metadata`] instead.
    ///
    /// This function is an async version of [`std::fs::symlink_metadata`].
    ///
    /// [`metadata`]: fn.metadata.html
    /// [`std::fs::symlink_metadata`]: https://doc.rust-lang.org/std/fs/fn.symlink_metadata.html
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `path` does not point to an existing file or directory.
    /// * The current process lacks permissions to read metadata for the path.
    /// * Some other I/O error occurred.
    ///
    /// # Examples
    ///
    /// Using the [`tokio`](https://docs.rs/tokio) runtime:
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> std::io::Result<()> {
    /// #
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::TokioFs;
    ///
    /// let meta = TokioFs::symlink_metadata("a.txt").await?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`async_std`](https://docs.rs/async-std) runtime:
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::AsyncStdFs;
    ///
    /// let meta = AsyncStdFs::symlink_metadata("a.txt").await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    async fn symlink_metadata<P: AsRef<Path> + Send>(path: P) -> std::io::Result<Metadata>;

    /// Writes a slice of bytes as the new contents of a file.
    ///
    /// This function will create a file if it does not exist, and will entirely replace its contents
    /// if it does.
    ///
    /// This function is an async version of [`std::fs::write`].
    ///
    /// [`std::fs::write`]: https://doc.rust-lang.org/std/fs/fn.write.html
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * The file's parent directory does not exist.
    /// * The current process lacks permissions to write to the file.
    /// * Some other I/O error occurred.
    ///
    /// # Examples
    ///
    /// Using the [`tokio`](https://docs.rs/tokio) runtime:
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> std::io::Result<()> {
    /// #
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::TokioFs;
    ///
    /// TokioFs::write("a.txt", b"Hello world!").await?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`async_std`](https://docs.rs/async-std) runtime:
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::AsyncStdFs;
    ///
    /// AsyncStdFs::write("a.txt", b"Hello world!").await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    async fn write<P: AsRef<Path> + Send, C: AsRef<[u8]> + Send>(
        path: P,
        contents: C
    ) -> std::io::Result<()>;
}



/// An async abstraction over [`std::fs::DirEntry`].
#[async_trait]
pub trait DirEntry {
    /// Returns the full path to this entry.
    ///
    /// The full path is created by joining the original path passed to [`read_dir`] with the name
    /// of this entry.
    ///
    /// [`read_dir`]: trait.Filesystem.html#tymethod.read_dir
    ///
    /// # Examples
    ///
    /// Using the [`tokio`](https://docs.rs/tokio) runtime:
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> std::io::Result<()> {
    /// #
    /// use futures::stream::StreamExt;
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::TokioFs;
    ///
    /// let mut entries = TokioFs::read_dir(".").await?;
    ///
    /// while let Some(res) = entries.next().await {
    ///     let entry = res?;
    ///     println!("{:?}", entry.path());
    /// }
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`async_std`](https://docs.rs/async-std) runtime:
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use futures::stream::StreamExt;
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::AsyncStdFs;
    ///
    /// let mut entries = AsyncStdFs::read_dir(".").await?;
    ///
    /// while let Some(res) = entries.next().await {
    ///     let entry = res?;
    ///     println!("{:?}", entry.path());
    /// }
    /// #
    /// # Ok(()) }) }
    /// ```
    fn path(&self) -> PathBuf;

    /// Returns the bare name of this entry without the leading path.
    ///
    /// # Examples
    ///
    /// Using the [`tokio`](https://docs.rs/tokio) runtime:
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> std::io::Result<()> {
    /// #
    /// use futures::stream::StreamExt;
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::TokioFs;
    ///
    /// let mut entries = TokioFs::read_dir(".").await?;
    ///
    /// while let Some(res) = entries.next().await {
    ///     let entry = res?;
    ///     println!("{}", entry.file_name().to_string_lossy());
    /// }
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`async_std`](https://docs.rs/async-std) runtime:
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use futures::stream::StreamExt;
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::AsyncStdFs;
    ///
    /// let mut entries = AsyncStdFs::read_dir(".").await?;
    ///
    /// while let Some(res) = entries.next().await {
    ///     let entry = res?;
    ///     println!("{}", entry.file_name().to_string_lossy());
    /// }
    /// #
    /// # Ok(()) }) }
    /// ```
    fn file_name(&self) -> OsString;

    /// Reads the metadata for this entry.
    ///
    /// This function will traverse symbolic links to read the metadata.
    ///
    /// If you want to read metadata without following symbolic links, use [`symlink_metadata`]
    /// instead.
    ///
    /// [`symlink_metadata`]: trait.Filesystem.html#tymethod.symlink_metadata
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * This entry does not point to an existing file or directory anymore.
    /// * The current process lacks permissions to read the metadata.
    /// * Some other I/O error occurred.
    ///
    /// # Examples
    ///
    /// Using the [`tokio`](https://docs.rs/tokio) runtime:
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> std::io::Result<()> {
    /// #
    /// use futures::stream::StreamExt;
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::TokioFs;
    ///
    /// let mut entries = TokioFs::read_dir(".").await?;
    ///
    /// while let Some(res) = entries.next().await {
    ///     let entry = res?;
    ///     println!("{:?}", entry.metadata().await?);
    /// }
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`async_std`](https://docs.rs/async-std) runtime:
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use futures::stream::StreamExt;
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::AsyncStdFs;
    ///
    /// let mut entries = AsyncStdFs::read_dir(".").await?;
    ///
    /// while let Some(res) = entries.next().await {
    ///     let entry = res?;
    ///     println!("{:?}", entry.metadata().await?);
    /// }
    /// #
    /// # Ok(()) }) }
    /// ```
    async fn metadata(&self) -> std::io::Result<Metadata>;

    /// Reads the file type for this entry.
    ///
    /// This function will not traverse symbolic links if this entry points at one.
    ///
    /// If you want to read metadata with following symbolic links, use [`metadata`] instead.
    ///
    /// [`metadata`]: #tymethod.metadata
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * This entry does not point to an existing file or directory anymore.
    /// * The current process lacks permissions to read this entry's metadata.
    /// * Some other I/O error occurred.
    ///
    /// # Examples
    ///
    /// Using the [`tokio`](https://docs.rs/tokio) runtime:
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> std::io::Result<()> {
    /// #
    /// use futures::stream::StreamExt;
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::TokioFs;
    ///
    /// let mut entries = TokioFs::read_dir(".").await?;
    ///
    /// while let Some(res) = entries.next().await {
    ///     let entry = res?;
    ///     println!("{:?}", entry.file_type().await?);
    /// }
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`async_std`](https://docs.rs/async-std) runtime:
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use futures::stream::StreamExt;
    /// use fut_compat::fs::Filesystem;
    /// use fut_compat::fs::AsyncStdFs;
    ///
    /// let mut entries = AsyncStdFs::read_dir(".").await?;
    ///
    /// while let Some(res) = entries.next().await {
    ///     let entry = res?;
    ///     println!("{:?}", entry.file_type().await?);
    /// }
    /// #
    /// # Ok(()) }) }
    /// ```
    async fn file_type(&self) -> std::io::Result<FileType>;
}



/// An async abstraction over [`std::fs::File`].
#[async_trait]
pub trait File: Sized {
    /// Opens a file in read-only mode.
    ///
    /// See the [`OpenOptions::open`] function for more options.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * `path` does not point to an existing file.
    /// * The current process lacks permissions to read the file.
    /// * Some other I/O error occurred.
    ///
    /// For more details, see the list of errors documented by [`OpenOptions::open`].
    ///
    /// [`OpenOptions::open`]: trait.OpenOptions.html#tymethod.open
    async fn open<P: AsRef<Path> + Send>(path: P) -> std::io::Result<Self>;

    /// Opens a file in write-only mode.
    ///
    /// This function will create a file if it does not exist, and will truncate it if it does.
    ///
    /// See the [`OpenOptions::open`] function for more options.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * The file's parent directory does not exist.
    /// * The current process lacks permissions to write to the file.
    /// * Some other I/O error occurred.
    ///
    /// For more details, see the list of errors documented by [`OpenOptions::open`].
    ///
    /// [`OpenOptions::open`]: trait.OpenOptions.html#tymethod.open
    async fn create<P: AsRef<Path> + Send>(path: P) -> std::io::Result<Self>;

    /// Synchronizes OS-internal buffered contents and metadata to disk.
    ///
    /// This function will ensure that all in-memory data reaches the filesystem.
    ///
    /// This can be used to handle errors that would otherwise only be caught when the file is
    /// closed. When a file is dropped, errors in synchronizing this in-memory data are ignored.
    async fn sync_all(&self) -> std::io::Result<()>;

    /// Synchronizes OS-internal buffered contents to disk.
    ///
    /// This is similar to [`sync_all`], except that file metadata may not be synchronized.
    ///
    /// This is intended for use cases that must synchronize the contents of the file, but don't
    /// need the file metadata synchronized to disk.
    ///
    /// Note that some platforms may simply implement this in terms of [`sync_all`].
    ///
    /// [`sync_all`]: #tymethod.sync_all
    async fn sync_data(&self) -> std::io::Result<()>;

    /// Truncates or extends the file.
    ///
    /// If `size` is less than the current file size, then the file will be truncated. If it is
    /// greater than the current file size, then the file will be extended to `size` and have all
    /// intermediate data filled with zeros.
    ///
    /// The file's cursor stays at the same position, even if the cursor ends up being past the end
    /// of the file after this operation.
    async fn set_len(&self, size: u64) -> std::io::Result<()>;

    /// Reads the file's metadata.
    async fn metadata(&self) -> std::io::Result<Metadata>;

    /// Changes the permissions on the file.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * The current process lacks permissions to change attributes on the file.
    /// * Some other I/O error occurred.
    async fn set_permissions(&self, perm: Permissions) -> std::io::Result<()>;
}

/// An async abstraction over [`std::fs::OpenOptions`].
///
/// A builder for opening files with configurable options.
///
/// Files can be opened in [`read`] and/or [`write`] mode.
///
/// The [`append`] option opens files in a special writing mode that moves the file cursor to the
/// end of file before every write operation.
///
/// It is also possible to [`truncate`] the file right after opening, to [`create`] a file if it
/// doesn't exist yet, or to always create a new file with [`create_new`].
///
/// [`read`]: #tymethod.read
/// [`write`]: #tymethod.write
/// [`append`]: #tymethod.append
/// [`truncate`]: #tymethod.truncate
/// [`create`]: #tymethod.create
/// [`create_new`]: #tymethod.create_new
/// [`std::fs::OpenOptions`]: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html
///
/// # Examples
///
/// Open a file for reading using the [`tokio`](https://docs.rs/tokio) runtime:
///
/// ```no_run
/// # #[tokio::main]
/// # async fn main() -> std::io::Result<()> {
/// #
/// use tokio::fs::OpenOptions;
///
/// let file = OpenOptions::new()
///     .read(true)
///     .open("a.txt")
///     .await?;
/// #
/// # Ok(())
/// # }
/// ```
///
/// Open a file for reading using the [`async_std`](https://docs.rs/async-std) runtime:
///
/// ```no_run
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::fs::OpenOptions;
///
/// let file = OpenOptions::new()
///     .read(true)
///     .open("a.txt")
///     .await?;
/// #
/// # Ok(()) }) }
/// ```
///
/// Open a file for both reading and writing, and create it if it doesn't exist yet
/// using the [`tokio`](https://docs.rs/tokio) runtime:
///
/// ```no_run
/// # #[tokio::main]
/// # async fn main() -> std::io::Result<()> {
/// #
/// use tokio::fs::OpenOptions;
///
/// let file = OpenOptions::new()
///     .read(true)
///     .write(true)
///     .create(true)
///     .open("a.txt")
///     .await?;
/// #
/// # Ok(())
/// # }
/// ```
///
/// Open a file for both reading and writing, and create it if it doesn't exist yet
/// using the [`async_std`](https://docs.rs/async-std) runtime:
///
/// ```no_run
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::fs::OpenOptions;
///
/// let file = OpenOptions::new()
///     .read(true)
///     .write(true)
///     .create(true)
///     .open("a.txt")
///     .await?;
/// #
/// # Ok(()) }) }
/// ```
#[async_trait]
pub trait OpenOptions: Sized {
    /// The file object which gets returned by the [`open`](#tymethod.open) method.
    type File: File;

    /// Creates a blank set of options.
    ///
    /// All options are initially set to `false`.
    fn new() -> Self;

    /// Configures the option for read mode.
    ///
    /// When set to `true`, this option means the file will be readable after opening.
    fn read(&mut self, read: bool) -> &mut Self;

    /// Configures the option for write mode.
    ///
    /// When set to `true`, this option means the file will be writable after opening.
    ///
    /// If the file already exists, write calls on it will overwrite the previous contents without
    /// truncating it.
    fn write(&mut self, write: bool) -> &mut Self;

    /// Configures the option for append mode.
    ///
    /// When set to `true`, this option means the file will be writable after opening and the file
    /// cursor will be moved to the end of file before every write operaiton.
    fn append(&mut self, append: bool) -> &mut Self;

    /// Configures the option for truncating the previous file.
    ///
    /// When set to `true`, the file will be truncated to the length of 0 bytes.
    ///
    /// The file must be opened in [`write`] or [`append`] mode for truncation to work.
    ///
    /// [`write`]: #tymethod.write
    /// [`append`]: #tymethod.append
    fn truncate(&mut self, truncate: bool) -> &mut Self;

    /// Configures the option for creating a new file if it doesn't exist.
    ///
    /// When set to `true`, this option means a new file will be created if it doesn't exist.
    ///
    /// The file must be opened in [`write`] or [`append`] mode for file creation to work.
    ///
    /// [`write`]: #tymethod.write
    /// [`append`]: #tymethod.append
    fn create(&mut self, create: bool) -> &mut Self;

    /// Configures the option for creating a new file or failing if it already exists.
    ///
    /// When set to `true`, this option means a new file will be created, or the open operation
    /// will fail if the file already exists.
    ///
    /// The file must be opened in [`write`] or [`append`] mode for file creation to work.
    ///
    /// [`write`]: #tymethod.write
    /// [`append`]: #tymethod.append
    fn create_new(&mut self, create_new: bool) -> &mut Self;

    /// Opens a file with the configured options.
    ///
    /// # Errors
    ///
    /// An error will be returned in the following situations:
    ///
    /// * The file does not exist and neither [`create`] nor [`create_new`] were set.
    /// * The file's parent directory does not exist.
    /// * The current process lacks permissions to open the file in the configured mode.
    /// * The file already exists and [`create_new`] was set.
    /// * Invalid combination of options was used, like [`truncate`] was set but [`write`] wasn't,
    ///   or none of [`read`], [`write`], and [`append`] modes was set.
    /// * An OS-level occurred, like too many files are open or the file name is too long.
    /// * Some other I/O error occurred.
    ///
    /// [`read`]: #tymethod.read
    /// [`write`]: #tymethod.write
    /// [`append`]: #tymethod.append
    /// [`truncate`]: #tymethod.truncate
    /// [`create`]: #tymethod.create
    /// [`create_new`]: #tymethod.create_new
    async fn open<P: AsRef<Path> + Send>(&self, path: P) -> std::io::Result<Self::File>;
}

/// An async abstraction over [`std::fs::DirBuilder`].
#[async_trait]
pub trait DirBuilder: Sized {
    /// Creates a blank set of options.
    ///
    /// The [`recursive`] option is initially set to `false`.
    ///
    /// [`recursive`]: #tymethod.recursive
    fn new() -> Self;

    /// Sets the option for recursive mode.
    ///
    /// When set to `true`, this option means all parent directories should be created recursively
    /// if they don't exist. Parents are created with the same permissions as the final directory.
    ///
    /// This option is initially set to `false`.
    fn recursive(&mut self, recursive: bool) -> &mut Self;

    /// Creates a directory with the configured options.
    ///
    /// It is considered an error if the directory already exists unless recursive mode is enabled.
    async fn create<P: AsRef<Path> + Send>(&self, path: P) -> std::io::Result<()>;
}
