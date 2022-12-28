use std::io;
use std::net::TcpListener;

use sqlx::{Connection, PgConnection};

use wpr::config;
use wpr::startup;

#[tokio::main]
async fn main() -> io::Result<()> {
    let config = config::load().expect("Config load error");
    let connection = PgConnection::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;

    startup::run(listener, connection)?.await
}
