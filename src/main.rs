use std::io;
use std::net::TcpListener;

use wpr::config;
use wpr::startup;

#[tokio::main]
async fn main() -> io::Result<()> {
    let config = config::load().expect("Config load error");

    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;

    startup::run(listener)?.await
}
