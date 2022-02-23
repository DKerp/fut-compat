//! Offers compatibility between the [`tokio`](https://docs.rs/tokio) and
//! [`async_std`](https://docs.rs/async-std) runtimes by providing their most
//! important functionalities through an unified interface.



/// Async abstractions over [`std::io`] and the implementations for the different runtimes.
pub mod io;

/// Async abstractions over [`std::fs`] and the implementations for the different runtimes.
pub mod fs;

/// Implementations for the different runtimes of the abstractions in [`futures::task`].
pub mod task;

/// Async abstractions over [`std::net`] and the implementations for the different runtimes.
pub mod net;
