use std::io::{Error, ErrorKind, SeekFrom};
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::io::{AsyncRead, AsyncBufRead, AsyncWrite, AsyncSeek};

pub use futures::io::AllowStdIo;



/// Contains the compatibility objects for the [`tokio`](https://docs.rs/tokio) runtime.
#[cfg(feature = "tokio")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "tokio")))]
mod tokio;
pub use self::tokio::*;
