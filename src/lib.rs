#![doc = include_str!("./lib.md")]

#![cfg_attr(docsrs, feature(doc_cfg))]



/// Async abstractions over [`std::io`] and the implementations for the different runtimes.
pub mod io;

/// Async abstractions over [`std::fs`] and the implementations for the different runtimes.
pub mod fs;

/// Implementations for the different runtimes of the abstractions in [`futures::task`].
pub mod task;

/// Async abstractions over [`std::net`] and the implementations for the different runtimes.
pub mod net;
