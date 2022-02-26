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
#[cfg(feature = "tokio")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "tokio")))]
mod tokio;
#[cfg(feature = "tokio")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "tokio")))]
pub use self::tokio::*;

/// Contains the compatibility objects for the [`async_std`](https://docs.rs/async-std) runtime.
#[cfg(feature = "async-std")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "async-std")))]
mod async_std;
#[cfg(feature = "async-std")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "async-std")))]
pub use self::async_std::*;



/// An async abstraction over [`std::os::unix::net::SocketAddr`].
#[cfg(unix)]
#[cfg_attr(doc_cfg, doc(cfg(unix)))]
pub trait UnixSocketAddr {
    fn is_unnamed(&self) -> bool;

    fn as_pathname(&self) -> Option<&Path>;
}

#[cfg(unix)]
#[cfg_attr(doc_cfg, doc(cfg(unix)))]
impl UnixSocketAddr for std::os::unix::net::SocketAddr {
    fn is_unnamed(&self) -> bool {
        self.is_unnamed()
    }

    fn as_pathname(&self) -> Option<&Path> {
        self.as_pathname()
    }
}



/// An async abstraction over [`std::net::ToSocketAddrs`].
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
    async fn connect<A: ToSocketAddrs + Send>(addrs: A) -> std::io::Result<Self>;

    async fn peek(&self, buf: &mut [u8]) -> std::io::Result<usize>;

    fn peer_addr(&self) -> std::io::Result<SocketAddr>;

    fn local_addr(&self) -> std::io::Result<SocketAddr>;

    fn nodelay(&self) -> std::io::Result<bool>;

    fn set_nodelay(&self, nodelay: bool) -> std::io::Result<()>;

    fn ttl(&self) -> std::io::Result<u32>;

    fn set_ttl(&self, ttl: u32) -> std::io::Result<()>;
}



/// An async abstraction over [`std::net::TcpListener`].
#[async_trait]
pub trait TcpListener: Sized {
    type TcpStream: TcpStream;

    async fn bind<A: ToSocketAddrs + Send>(addrs: A) -> std::io::Result<Self>;

    async fn accept(&self) -> std::io::Result<(Self::TcpStream, SocketAddr)>;

    fn local_addr(&self) -> std::io::Result<SocketAddr>;
}




/// An async abstraction over [`std::os::unix::net::UnixStream`].
#[cfg(unix)]
#[cfg_attr(doc_cfg, doc(cfg(unix)))]
#[async_trait]
pub trait UnixStream: Sized {
    type SocketAddr: UnixSocketAddr;

    async fn connect<P: AsRef<Path> + Send>(path: P) -> std::io::Result<Self>;

    fn pair() -> std::io::Result<(Self, Self)>;

    fn peer_addr(&self) -> std::io::Result<Self::SocketAddr>;

    fn local_addr(&self) -> std::io::Result<Self::SocketAddr>;
}



/// An async abstraction over [`std::os::unix::net::UnixListener`].
#[cfg(unix)]
#[cfg_attr(doc_cfg, doc(cfg(unix)))]
#[async_trait]
pub trait UnixListener: Sized {
    type UnixStream: UnixStream;
    type SocketAddr: UnixSocketAddr;

    async fn bind<P: AsRef<Path> + Send>(path: P) -> std::io::Result<Self>;

    async fn accept(&self) -> std::io::Result<(Self::UnixStream, Self::SocketAddr)>;

    fn local_addr(&self) -> std::io::Result<Self::SocketAddr>;
}
