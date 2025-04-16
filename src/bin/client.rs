use std::net::SocketAddr;

use tokio_uring::net::UdpSocket;
use udp_server::{Message, MessageInfo, get_server_addr, gzip_decode, message_encode, recv};

fn main() {
    dotenvy::dotenv();
    println!("-- Client --");
    tokio_uring::start(async {
        let server_addr: SocketAddr = get_server_addr();
        let socket: UdpSocket = UdpSocket::bind("0.0.0.0:0".parse().unwrap()).await.unwrap();
        println!("{}", socket.local_addr().unwrap());
        socket
            .send_to(
                message_encode(Message {
                    user_id: 1,
                    message: String::from("Hello World!"),
                }),
                server_addr.clone(),
            )
            .await;

        if let Some((_, buf)) = recv(&socket).await {
            let gzip_decoded = gzip_decode(&buf);
            let message_info: MessageInfo = serde_json::from_slice(&gzip_decoded).unwrap();
            println!("{:?}", message_info);
        }
    });
}
