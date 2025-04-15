use std::net::SocketAddr;

use tokio_uring::net::UdpSocket;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;
    println!("-- Client --");
    tokio_uring::start(async {
        let server_addr: SocketAddr = udp_server::get_server_addr()?;
        let client_socket: UdpSocket = UdpSocket::bind("0.0.0.0:0".parse().unwrap()).await?;
        println!("{}", client_socket.local_addr().unwrap());
        let message = Vec::from("behzad");

        let (result, _buf) = client_socket
            .send_to(udp_server::gzip_encode(&message), server_addr.clone())
            .await;
        result.unwrap();
        Ok(())
    })
}
