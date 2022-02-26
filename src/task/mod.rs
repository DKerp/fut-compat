use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::error::Error;

pub use futures::task::{Spawn, SpawnExt};



/// Contains the compatibility objects for the [`tokio`](https://docs.rs/tokio) runtime.
#[cfg(feature = "tokio-rt")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio-rt")))]
mod tokio;
#[cfg(feature = "tokio-rt")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio-rt")))]
pub use self::tokio::*;

/// Contains the compatibility objects for the [`async_std`](https://docs.rs/async-std) runtime.
#[cfg(feature = "async-std-rt")]
#[cfg_attr(docsrs, doc(cfg(feature = "async-std-rt")))]
mod async_std;
#[cfg(feature = "async-std-rt")]
#[cfg_attr(docsrs, doc(cfg(feature = "async-std-rt")))]
pub use self::async_std::*;



/// An abstraction over executing a sync task in a new blocking thread and optionally awaiting
/// it's completion in an async fashion.
pub trait SpawnBlocking {
    fn spawn_blocking<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static;
}


/// A handle that awaits the result of a task. Gets returned by [`SpawnBlocking`].
pub struct JoinHandle<T> {
    inner: Box<dyn Future<Output = Result<T, Box<dyn Error>>> + Unpin +'static>
}

impl<T> JoinHandle<T>
{
    pub fn new<J>(inner: J) -> Self
    where
        J: Future<Output = Result<T, Box<dyn Error>>> + Unpin + 'static,
    {
        Self {
            inner: Box::new(inner),
        }
    }
}

impl<T> Future for JoinHandle<T>
{
    type Output = Result<T, Box<dyn Error>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Future::poll(Pin::new(&mut Pin::into_inner(self).inner), cx)
    }
}
