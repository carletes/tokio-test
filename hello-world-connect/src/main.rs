use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::net::TcpStream;

#[tokio::main]

async fn main() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 6142);
    let _stream = TcpStream::connect(addr).await;
}
