use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;

pub fn get_server_addr() -> Result<SocketAddr, Box<dyn std::error::Error>> {
    let host: String = env::var("APP_HOST")?;
    let port: u16 = env::var("APP_PORT")?.parse()?;
    Ok(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::from_str(&host)?),
        port,
    ))
}
