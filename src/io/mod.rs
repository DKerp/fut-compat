pub use std::io::Result;

pub use futures::io::AllowStdIo;
pub use futures::io::{BufReader, BufWriter};

pub use futures::io::{
    AsyncRead,
    AsyncReadExt,
    AsyncBufRead,
    AsyncBufReadExt,
    AsyncWrite,
    AsyncWriteExt,
    AsyncSeek,
    AsyncSeekExt,
};

pub use futures::io::{
    copy,
    copy_buf,
    empty,
    repeat,
    sink,
};





/// Contains the compatibility objects for the [`tokio`](https://docs.rs/tokio) runtime.
#[cfg(feature = "tokio")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "tokio")))]
mod tokio;
#[cfg(feature = "tokio")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "tokio")))]
pub use self::tokio::*;
