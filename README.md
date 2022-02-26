# fut-compat

[![Crates.io](https://img.shields.io/crates/v/fut-compat)](https://crates.io/crates/fut-compat)
[![docs.rs](https://img.shields.io/docsrs/fut-compat)](https://docs.rs/fut-compat)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/DKerp/fut-compat/blob/main/LICENSE)

Offers compatibility between the [`tokio`](https://docs.rs/tokio) and [`async_std`](https://docs.rs/async-std) runtimes by providing their most important functionalities through an unified interface.

The goal of this crate is to enable the developement of async crates which can make use of all the important objects (e.g. `File`, `TcpListener`, etc.) and functions (e.g. `spawn` for spawning new tasks) provided by fully-fledged async runtimes without relying on a concrete runtime. This enables downstream developers to choose the runtime which fits their needs best.

This is archieved by providing abstractions over all important functionalities and implementing them for the respective runtimes by using their respective objects and methods. We either make us of traits already given by the [`futures`](https://docs.rs/futures) crate maintained by the rust team, or we define our own.

## Features

By default no runtime specific implementation is included to reduce the number of crates you have to import. In order to use a specific runtime you must activate the corresponding feature:

|Feature|Runtime|
|---------|--------|
| `tokio-rt` | [`tokio`](https://docs.rs/tokio) |
| `async-std-rt` | [`async_std`](https://docs.rs/async-std) |

## Example

The [`futures`](https://docs.rs/futures) crate defines the [`Spawn`](https://docs.rs/futures/latest/futures/task/trait.Spawn.html) trait which abstracts away the `spawn` methods provided by the [`tokio`](https://docs.rs/tokio) and [`async_std`](https://docs.rs/async-std) runtimes which are used to spawn new tasks.

This crate provides two objects `TokioExecutor` and `AsyncStdExecutor` which implement the [`Spawn`](https://docs.rs/futures/latest/futures/task/trait.Spawn.html) trait by internally calling the `spawn` method of the respective runtime. Now you can write code which internally requires a way of spawning new tasks without relying on a particular runtime.

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

## TODO

- Task management
  - [x] Make it possible to spawn new tasks.
  - [x] Make it possible to spawn new blocking tasks.
  - [ ] Make it possible to abort tasks.
  - [ ] Provide gracefull handling of panicked tasks. (asnyc-std does not seem to support this, unlike tokio)
- IO
  - [x] Provide compatibility between tokio objects and the futures io traits (`AsyncRead` etc.).
  - [x] Provide compatibility between sync io traits and async io traits. (Re-exported `AllowStdIo` from `futures::io`)
  - [x] Provide buffered reading/writing. (Re-exported `BufReader` and `BufWriter` from `futures::io`)
  - [x] Provide common helper functions. (Re-exported them from `futures::io`)
  - [ ] Add a `copy_bidirectional` helper function.
  - [ ] Provide async access to `Stdout`/`Stdin`/`Stderr`.
- Filesystem
  - [x] Provide access to all common utility functions (e.g. `create_dir`, `read_to_string` etc.).
  - [x] Provide a common interface to reading directories.
  - [x] Provide a common `File` interface.
  - [x] Provide a common interface for opening `File`s with custom options.
- Networking
  - [x] Provide common `TcpListener`/`TcpSocket` interfaces.
  - [x] Provide common `UnixListener`/`UnixSocket` interfaces.
  - [ ] Provide a common `UdpSocket` interface.
- Process management
  - [ ] Make it possible to spawn new processes.
  - [ ] Make it possible to abort spawned processes.
  - [ ] Provide a common `Command` interface.
  - [ ] Provide async access to `Stdout`/`Stdin`/`Stderr` of started processes.
  - [ ] Reading/sending signals.
- Documentation
  - [x] Provide basic documentation.
  - [ ] Provide a more detailed documentation.
  - [ ] Add examples everywhere.

## License

This library is licensed under the [MIT license](http://opensource.org/licenses/MIT).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this library by you, shall be licensed as MIT, without any additional terms or conditions.
