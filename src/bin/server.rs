use tokio_uring::net::UdpSocket;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;
    println!("-- Server --");
    tokio_uring::start(async {
        let socket: UdpSocket = UdpSocket::bind(udp_server::get_server_addr()?).await?;
        loop {
            let buf = vec![0u8; 1024];
            let (result, mut buf) = socket.recv_from(buf).await;
            if let Ok((read, client_addr)) = result {
                buf.resize(read, 0);
                println!(
                    "recv \"{}\" from {}",
                    String::from_utf8_lossy(&buf),
                    client_addr
                );
            }
        }
    })
}
