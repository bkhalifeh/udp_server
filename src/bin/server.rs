use std::{net::UdpSocket, os::fd::AsRawFd, thread};

use udp_server::{get_server_addr, worker_impl};

fn main() {
    dotenvy::dotenv().unwrap();
    println!("-- Server --");

    let num_cores = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    let mut workers = Vec::new();

    let server_addr: std::net::SocketAddr = get_server_addr();
    let socket = UdpSocket::bind(server_addr).unwrap();
    socket.set_nonblocking(true).unwrap();
    let socket_fd = socket.as_raw_fd();
    for i in 0..num_cores {
        let worker = thread::spawn(move || {
            tokio_uring::start(worker_impl(socket_fd, i));
        });
        workers.push(worker);
    }
    for worker in workers {
        worker.join().expect("Worker stopped");
    }
}
