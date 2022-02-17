use super::*;



/// An executor for the [`tokio`](https://docs.rs/tokio) runtime.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TokioExecutor {}

impl Spawn for TokioExecutor {
    fn spawn_obj(
        &self,
        future: FutureObj<'static, ()>
    ) -> Result<(), SpawnError> {
        ::tokio::task::spawn(future);

        Ok(())
    }
}

impl LocalSpawn for TokioExecutor {
    fn spawn_local_obj(
        &self,
        future: LocalFutureObj<'static, ()>
    ) -> Result<(), SpawnError> {
        ::tokio::task::spawn_local(future);

        Ok(())
    }
}

impl SpawnBlocking for TokioExecutor {
    fn spawn_blocking<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let fut = ::tokio::task::spawn_blocking::<F, T>(f);
        let fut = FutureExt::map(fut, |result| result.map_err(|err| {
            let box_err: Box<dyn Error> = Box::new(err);

            box_err
        }));

        JoinHandle::new(fut)
    }
}
