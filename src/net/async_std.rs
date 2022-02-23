use super::*;

use ::async_std::net;



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
#[async_trait]
impl UnixStream for ::async_std::os::unix::net::UnixStream {
    type SocketAddr = std::os::unix::net::SocketAddr;

    async fn connect<P: AsRef<Path> + Send>(path: P) -> std::io::Result<Self> {
        let path = path.as_ref();
        let path: &Path = path.into();
        
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
