use super::*;

use futures::task::{Spawn, LocalSpawn};
use futures::task::{SpawnError, FutureObj, LocalFutureObj};
use futures::FutureExt;



/// An executor for the [`async_std`](https://docs.rs/async-std) runtime.
#[cfg(feature = "async-std-rt")]
#[cfg_attr(docsrs, doc(cfg(feature = "async-std-rt")))]
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AsyncStdExecutor {}

impl Spawn for AsyncStdExecutor {
    fn spawn_obj(
        &self,
        future: FutureObj<'static, ()>
    ) -> Result<(), SpawnError> {
        ::async_std::task::spawn(future);

        Ok(())
    }
}

impl LocalSpawn for AsyncStdExecutor {
    fn spawn_local_obj(
        &self,
        future: LocalFutureObj<'static, ()>
    ) -> Result<(), SpawnError> {
        ::async_std::task::spawn_local(future);

        Ok(())
    }
}

impl SpawnBlocking for AsyncStdExecutor {
    fn spawn_blocking<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let fut = ::async_std::task::spawn_blocking::<F, T>(f);
        let fut = FutureExt::map(fut, |ret| {
            let result: Result<T, Box<dyn Error>> = Ok(ret);

            result
        });

        JoinHandle::new(fut)
    }
}
