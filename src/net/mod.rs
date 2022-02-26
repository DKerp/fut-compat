use std::net::{
    SocketAddr,
    SocketAddrV4,
    SocketAddrV6,
    IpAddr,
};
use std::str::FromStr;
use std::path::Path;

use async_trait::async_trait;



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



/// An async abstraction over [`std::os::unix::net::SocketAddr`].
#[cfg(unix)]
#[cfg_attr(docsrs, doc(cfg(unix)))]
pub trait UnixSocketAddr {
    /// Returns `true` if the address is unnamed.
    fn is_unnamed(&self) -> bool;

    /// Returns the contents of this address if it is a `pathname` address.
    fn as_pathname(&self) -> Option<&Path>;
}

#[cfg(unix)]
#[cfg_attr(docsrs, doc(cfg(unix)))]
impl UnixSocketAddr for std::os::unix::net::SocketAddr {
    fn is_unnamed(&self) -> bool {
        self.is_unnamed()
    }

    fn as_pathname(&self) -> Option<&Path> {
        self.as_pathname()
    }
}



/// An async abstraction over [`std::net::ToSocketAddrs`].
///
/// Converts or resolves addresses to [`SocketAddr`] values.
#[async_trait]
pub trait ToSocketAddrs {
    type Iter: Iterator<Item = SocketAddr>;

    async fn to_socket_addrs(self) -> Self::Iter;
}

#[async_trait]
impl<I> ToSocketAddrs for (I, u16)
where
    I: Into<IpAddr> + Send,
{
    type Iter = std::array::IntoIter<SocketAddr, 1>;

    async fn to_socket_addrs(self) -> Self::Iter {
        let addr: SocketAddr = self.into();

        IntoIterator::into_iter([addr])
    }
}

#[async_trait]
impl ToSocketAddrs for SocketAddrV4 {
    type Iter = std::array::IntoIter<SocketAddr, 1>;

    async fn to_socket_addrs(self) -> Self::Iter {
        let addr: SocketAddr = self.into();

        IntoIterator::into_iter([addr])
    }
}

#[async_trait]
impl ToSocketAddrs for SocketAddrV6 {
    type Iter = std::array::IntoIter<SocketAddr, 1>;

    async fn to_socket_addrs(self) -> Self::Iter {
        let addr: SocketAddr = self.into();

        IntoIterator::into_iter([addr])
    }
}

#[async_trait]
impl ToSocketAddrs for String {
    type Iter = std::vec::IntoIter<SocketAddr>;

    async fn to_socket_addrs(self) -> Self::Iter {
        let addr: Vec<SocketAddr> = match SocketAddr::from_str(&self) {
            Ok(addr) => vec![addr],
            Err(_) => Vec::new(),
        };

        IntoIterator::into_iter(addr)
    }
}

#[async_trait]
impl ToSocketAddrs for &str {
    type Iter = std::vec::IntoIter<SocketAddr>;

    async fn to_socket_addrs(self) -> Self::Iter {
        let addr: Vec<SocketAddr> = match SocketAddr::from_str(self) {
            Ok(addr) => vec![addr],
            Err(_) => Vec::new(),
        };

        IntoIterator::into_iter(addr)
    }
}

#[async_trait]
impl ToSocketAddrs for &[SocketAddr] {
    type Iter = std::vec::IntoIter<SocketAddr>;

    async fn to_socket_addrs(self) -> Self::Iter {
        let addr: Vec<SocketAddr> = self.iter().map(|&addr| addr).collect();

        IntoIterator::into_iter(addr)
    }
}



/// An async abstraction over [`std::net::TcpStream`].
#[async_trait]
pub trait TcpStream: Sized {
    /// Opens a TCP connection to a remote host.
    ///
    /// `addrs` is an address of the remote host. Anything which implements the
    /// [`ToSocketAddrs`] trait can be supplied as the address.  If `addrs`
    /// yields multiple addresses, connect will be attempted with each of the
    /// addresses until a connection is successful. If none of the addresses
    /// result in a successful connection, the error returned from the last
    /// connection attempt (the last address) is returned.
    async fn connect<A: ToSocketAddrs + Send>(addrs: A) -> std::io::Result<Self>;

    /// Receives data on the socket from the remote address to which it is connected, without
    /// removing that data from the queue.
    ///
    /// On success, returns the number of bytes peeked.
    ///
    /// Successive calls return the same data. This is accomplished by passing `MSG_PEEK` as a flag
    /// to the underlying `recv` system call.
    async fn peek(&self, buf: &mut [u8]) -> std::io::Result<usize>;

    /// Returns the remote address that this stream is connected to.
    fn peer_addr(&self) -> std::io::Result<SocketAddr>;

    /// Returns the local address that this stream is connected to.
    fn local_addr(&self) -> std::io::Result<SocketAddr>;

    /// Gets the value of the `TCP_NODELAY` option on this socket.
    ///
    /// For more information about this option, see [`set_nodelay`].
    ///
    /// [`set_nodelay`]: #tymethod.set_nodelay
    fn nodelay(&self) -> std::io::Result<bool>;

    /// Sets the value of the `TCP_NODELAY` option on this socket.
    ///
    /// If set, this option disables the Nagle algorithm. This means that
    /// segments are always sent as soon as possible, even if there is only a
    /// small amount of data. When not set, data is buffered until there is a
    /// sufficient amount to send out, thereby avoiding the frequent sending of
    /// small packets.
    fn set_nodelay(&self, nodelay: bool) -> std::io::Result<()>;

    /// Gets the value of the `IP_TTL` option for this socket.
    ///
    /// For more information about this option, see [`set_ttl`].
    ///
    /// [`set_ttl`]: #tymethod.set_ttl
    fn ttl(&self) -> std::io::Result<u32>;

    /// Sets the value for the `IP_TTL` option on this socket.
    ///
    /// This value sets the time-to-live field that is used in every packet sent
    /// from this socket.
    fn set_ttl(&self, ttl: u32) -> std::io::Result<()>;
}



/// An async abstraction over [`std::net::TcpListener`].
#[async_trait]
pub trait TcpListener: Sized {
    type TcpStream: TcpStream;

    /// Creates a new `TcpListener` which will be bound to the specified address.
    ///
    /// The returned listener is ready for accepting connections.
    ///
    /// Binding with a port number of 0 will request that the OS assigns a port to this listener.
    /// The port allocated can be queried via the [`local_addr`] method.
    ///
    /// [`local_addr`]: #tymethod.local_addr
    async fn bind<A: ToSocketAddrs + Send>(addrs: A) -> std::io::Result<Self>;

    /// Accepts a new incoming connection to this listener.
    ///
    /// When a connection is established, the corresponding stream and address will be returned.
    async fn accept(&self) -> std::io::Result<(Self::TcpStream, SocketAddr)>;

    /// Returns the local address that this listener is bound to.
    ///
    /// This can be useful, for example, to identify when binding to port 0 which port was assigned
    /// by the OS.
    fn local_addr(&self) -> std::io::Result<SocketAddr>;
}




/// An async abstraction over [`std::os::unix::net::UnixStream`].
#[cfg(unix)]
#[cfg_attr(docsrs, doc(cfg(unix)))]
#[async_trait]
pub trait UnixStream: Sized {
    type SocketAddr: UnixSocketAddr;

    /// Connects to the socket to the specified address.
    async fn connect<P: AsRef<Path> + Send>(path: P) -> std::io::Result<Self>;

    /// Creates an unnamed pair of connected sockets.
    ///
    /// Returns two streams which are connected to each other.
    fn pair() -> std::io::Result<(Self, Self)>;

    /// Returns the socket address of the local half of this connection.
    fn peer_addr(&self) -> std::io::Result<Self::SocketAddr>;

    /// Returns the socket address of the remote half of this connection.
    fn local_addr(&self) -> std::io::Result<Self::SocketAddr>;
}



/// An async abstraction over [`std::os::unix::net::UnixListener`].
#[cfg(unix)]
#[cfg_attr(docsrs, doc(cfg(unix)))]
#[async_trait]
pub trait UnixListener: Sized {
    type UnixStream: UnixStream;
    type SocketAddr: UnixSocketAddr;

    /// Creates a new unix listener bound to the specified path.
    async fn bind<P: AsRef<Path> + Send>(path: P) -> std::io::Result<Self>;

    /// Accepts a new incoming connection to this listener.
    ///
    /// When a connection is established, the corresponding stream and address will be returned.
    async fn accept(&self) -> std::io::Result<(Self::UnixStream, Self::SocketAddr)>;

    /// Returns the local socket address of this listener.
    fn local_addr(&self) -> std::io::Result<Self::SocketAddr>;
}
