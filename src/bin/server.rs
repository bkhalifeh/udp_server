fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;
    println!("-- Server --");
    tokio_uring::start(async {
        let socket = udp_server::setup_socket().await?;

        Ok(())
    })
}
