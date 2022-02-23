Offers compatibility between the [`tokio`](https://docs.rs/tokio) and
[`async_std`](https://docs.rs/async-std) runtimes by providing their most
important functionalities through an unified interface.

The goal of this crate is to enable the developement of async crates which can make use of all the important objects (e.g. `File`, `TcpListener`, etc.) and functions (e.g. `spawn` for spawning new tasks) provided by fully-fledged runtimes without relying on a concrete runtime. This enables downstream developers to choose the runtime which fits their needs best.

This is archieved by providing abstractions over all important functionalities and implementing them for the respective runtimes by using their respective objects and methods. We either make us of traits already given by the [`futures`](https://docs.rs/futures) crate maintained by the rust team, or we define our own.

# Features

By default no runtime specific implementation is included to reduce the number of crates
you have to import. In order to use a specific runtime you must activate the corresponding
feature:

|Feature|Runtime|
|---------|--------|
| `tokio-rt` | [`tokio`](https://docs.rs/tokio) |
| `async-std-rt` | [`async_std`](https://docs.rs/async-std) |

# Example

The [`futures`](https://docs.rs/futures) crate defines the [`Spawn`](https://docs.rs/futures/latest/futures/task/trait.Spawn.html) trait which abstracts away the `spawn` methods provided by the [`tokio`](https://docs.rs/tokio) and
[`async_std`](https://docs.rs/async-std) runtimes which are used to spawn new tasks.

This crate provides two objects [`TokioExecutor`](crate::task::TokioExecutor) and [`AsyncStdExecutor`](crate::task::AsyncStdExecutor) which implement the [`Spawn`](https://docs.rs/futures/latest/futures/task/trait.Spawn.html) trait by internally calling the `spawn` method of the respective runtime. Now you can write code which internally requires a way of spawning new tasks without relying on a particular runtime.

```rust
use futures::task::{Spawn, SpawnExt, SpawnError};
// Replace with AsyncStdExecutor for making use of the async_std runtime instead.
use fut_compat::task::TokioExecutor;


struct MustStartBackgroundTasks<E> {
    executor: E,
}

impl<E> MustStartBackgroundTasks<E>
where
    E: Spawn,
{
    async fn add(&self, x: u64, y: u64) -> Result<u64, SpawnError> {
        // Start a new task, just like you would with e.g. `tokio::spawn`.
        // `spawn_with_handle` must be used if you want to get a return value
        // from the task.
        let handle = self.executor.spawn_with_handle(async move {
            let z = x + y;

            z
        })?;

        let z = handle.await;

        Ok(z)
    }
}

// Replace with [async_std::main] if needed so.
#[tokio::main]
async fn main() {
    let executor = TokioExecutor::default();

    let main_object = MustStartBackgroundTasks {
        executor,
    };

    let z = main_object.add(2, 2).await.unwrap();

    println!("2 + 2 = {}", z);
}

```
