use std::io;
use std::net::TcpListener;

const ADDR: &str = "127.0.0.1:8000";

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = std::env::args().nth(1).unwrap_or_else(|| ADDR.to_string());
    let listener = TcpListener::bind(addr)?;

    wpr::startup::run(listener)?.await
}
