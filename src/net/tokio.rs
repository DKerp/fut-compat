use super::*;
use crate::io::TokioCompat;

use ::tokio::net;



#[cfg(unix)]
#[cfg_attr(docsrs, doc(cfg(unix)))]
impl UnixSocketAddr for net::unix::SocketAddr {
    fn is_unnamed(&self) -> bool {
        self.is_unnamed()
    }

    fn as_pathname(&self) -> Option<&Path> {
        self.as_pathname()
    }
}



#[async_trait]
impl TcpStream for net::TcpStream {
    async fn connect<A: ToSocketAddrs + Send>(addrs: A) -> std::io::Result<Self> {
        let addrs: Vec<SocketAddr> = ToSocketAddrs::to_socket_addrs(addrs).await.collect();

        Self::connect(&addrs[..]).await
    }

    async fn peek(&self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.peek(buf).await
    }

    fn peer_addr(&self) -> std::io::Result<SocketAddr> {
        self.peer_addr()
    }

    fn local_addr(&self) -> std::io::Result<SocketAddr> {
        self.local_addr()
    }

    fn nodelay(&self) -> std::io::Result<bool> {
        self.nodelay()
    }

    fn set_nodelay(&self, nodelay: bool) -> std::io::Result<()> {
        self.set_nodelay(nodelay)
    }

    fn ttl(&self) -> std::io::Result<u32> {
        self.ttl()
    }

    fn set_ttl(&self, ttl: u32) -> std::io::Result<()> {
        self.set_ttl(ttl)
    }
}

#[async_trait]
impl TcpStream for TokioCompat<net::TcpStream> {
    async fn connect<A: ToSocketAddrs + Send>(addrs: A) -> std::io::Result<Self> {
        let addrs: Vec<SocketAddr> = ToSocketAddrs::to_socket_addrs(addrs).await.collect();

        let inner = net::TcpStream::connect(&addrs[..]).await?;

        Ok(Self::new(inner))
    }

    async fn peek(&self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.get_ref().peek(buf).await
    }

    fn peer_addr(&self) -> std::io::Result<SocketAddr> {
        self.get_ref().peer_addr()
    }

    fn local_addr(&self) -> std::io::Result<SocketAddr> {
        self.get_ref().local_addr()
    }

    fn nodelay(&self) -> std::io::Result<bool> {
        self.get_ref().nodelay()
    }

    fn set_nodelay(&self, nodelay: bool) -> std::io::Result<()> {
        self.get_ref().set_nodelay(nodelay)
    }

    fn ttl(&self) -> std::io::Result<u32> {
        self.get_ref().ttl()
    }

    fn set_ttl(&self, ttl: u32) -> std::io::Result<()> {
        self.get_ref().set_ttl(ttl)
    }
}



#[async_trait]
impl TcpListener for net::TcpListener {
    type TcpStream = net::TcpStream;

    async fn bind<A: ToSocketAddrs + Send>(addrs: A) -> std::io::Result<Self> {
        let addrs: Vec<SocketAddr> = ToSocketAddrs::to_socket_addrs(addrs).await.collect();

        Self::bind(&addrs[..]).await
    }

    async fn accept(&self) -> std::io::Result<(Self::TcpStream, SocketAddr)> {
        self.accept().await
    }

    fn local_addr(&self) -> std::io::Result<SocketAddr> {
        self.local_addr()
    }
}



#[cfg(unix)]
#[cfg_attr(docsrs, doc(cfg(unix)))]
#[async_trait]
impl UnixStream for net::UnixStream {
    type SocketAddr = net::unix::SocketAddr;

    async fn connect<P: AsRef<Path> + Send>(path: P) -> std::io::Result<Self> {
        Self::connect(path).await
    }

    fn pair() -> std::io::Result<(Self, Self)> {
        Self::pair()
    }

    fn peer_addr(&self) -> std::io::Result<Self::SocketAddr> {
        self.peer_addr()
    }

    fn local_addr(&self) -> std::io::Result<Self::SocketAddr> {
        self.local_addr()
    }
}

#[cfg(unix)]
#[cfg_attr(docsrs, doc(cfg(unix)))]
#[async_trait]
impl UnixStream for TokioCompat<net::UnixStream> {
    type SocketAddr = net::unix::SocketAddr;

    async fn connect<P: AsRef<Path> + Send>(path: P) -> std::io::Result<Self> {
        let inner = net::UnixStream::connect(path).await?;

        Ok(Self::new(inner))
    }

    fn pair() -> std::io::Result<(Self, Self)> {
        net::UnixStream::pair().map(|(inner1, inner2)| (Self::new(inner1), Self::new(inner2)))
    }

    fn peer_addr(&self) -> std::io::Result<Self::SocketAddr> {
        self.get_ref().peer_addr()
    }

    fn local_addr(&self) -> std::io::Result<Self::SocketAddr> {
        self.get_ref().local_addr()
    }
}



#[cfg(unix)]
#[cfg_attr(docsrs, doc(cfg(unix)))]
#[async_trait]
impl UnixListener for net::UnixListener {
    type UnixStream = net::UnixStream;
    type SocketAddr = net::unix::SocketAddr;

    async fn bind<P: AsRef<Path> + Send>(path: P) -> std::io::Result<Self> {
        Self::bind(path)
    }

    async fn accept(&self) -> std::io::Result<(Self::UnixStream, Self::SocketAddr)> {
        self.accept().await
    }

    fn local_addr(&self) -> std::io::Result<Self::SocketAddr> {
        self.local_addr()
    }
}
