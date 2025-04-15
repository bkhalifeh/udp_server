use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use tokio_uring::net::UdpSocket;

pub async fn setup_socket() -> Result<UdpSocket, Box<dyn std::error::Error>> {
    let host: String = env::var("APP_HOST")?;
    let port: u16 = env::var("APP_PORT")?.parse()?;
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::from_str(&host)?), port);

    Ok(UdpSocket::bind(addr.clone()).await?)
}
