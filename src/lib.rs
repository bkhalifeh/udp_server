use diesel::prelude::*;

use diesel_async::RunQueryDsl;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::bb8::Pool;
use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;

use serde::{Deserialize, Serialize};

use std::env;
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::os::fd::FromRawFd;
use std::str::FromStr;
use std::time::SystemTime;

use tokio_uring::net::UdpSocket;

pub mod schema;

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[diesel(table_name = crate::schema::messages)]
pub struct Message {
    pub user_id: i32,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MessageInfo {
    pub id: i32,
    pub user_id: i32,
    pub message: String,
    pub created_at: SystemTime,
}

pub fn get_server_addr() -> SocketAddr {
    let host: String = env::var("APP_HOST").unwrap();
    let port: u16 = env::var("APP_PORT").unwrap().parse().unwrap();
    SocketAddr::new(IpAddr::V4(Ipv4Addr::from_str(&host).unwrap()), port)
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

pub fn message_encode<T: Serialize>(m: T) -> Vec<u8> {
    let json = serde_json::to_vec(&m).unwrap();
    gzip_encode(&json)
}

pub fn message_decode<T: for<'a> Deserialize<'a>>(encoded_message: &[u8]) -> T {
    let json = gzip_decode(&encoded_message);
    serde_json::from_slice(&json).unwrap()
}

pub async fn recv(s: &UdpSocket) -> Option<(SocketAddr, Vec<u8>)> {
    let buf = vec![0u8; 1024];
    let (result, mut buf) = s.recv_from(buf).await;
    if let Ok((read, client_addr)) = result {
        buf.resize(read, 0);
        return Some((client_addr, buf));
    } else {
        return None;
    }
}

pub async fn worker_impl(socket_fd: i32, worker_id: usize) {
    println!("-- Worker started with id {worker_id}");
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(
        std::env::var("DATABASE_URL").unwrap(),
    );
    let pool = Pool::builder().build(config).await.unwrap();

    let socket = unsafe { UdpSocket::from_raw_fd(socket_fd) };

    loop {
        if let Some((client_addr, buf)) = recv(&socket).await {
            let new_message: Message = message_decode(&buf);
            let mut conn = pool.get().await.unwrap();
            let message_info = diesel::insert_into(schema::messages::table)
                .values(&new_message)
                .returning(MessageInfo::as_returning())
                .get_result(&mut conn)
                .await
                .unwrap();

            socket
                .send_to(message_encode(message_info), client_addr)
                .await;
        }
    }
}
