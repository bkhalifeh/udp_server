use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use serde::{Deserialize, Serialize};
use std::env;
use std::io::{Read, Write};
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

pub fn gzip_encode(buf: &[u8]) -> Vec<u8> {
    let mut e: GzEncoder<Vec<u8>> = GzEncoder::new(Vec::new(), Compression::default());
    e.write(buf).unwrap();
    e.finish().unwrap()
}

pub fn gzip_decode(buf: &[u8]) -> Vec<u8> {
    let mut d = GzDecoder::new(buf);
    let mut out = Vec::new();
    d.read_to_end(&mut out).unwrap();
    out
}

pub fn message_encode(m: Message) -> Vec<u8> {
    let json = serde_json::to_vec(&m).unwrap();
    gzip_encode(&json)
}

pub fn message_decode(encoded_message: &[u8]) -> Message {
    let json = gzip_decode(&encoded_message);
    serde_json::from_slice(&json).unwrap()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub user_id: i32,
    pub message: String,
}
