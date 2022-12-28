use std::io;
use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgConnection;

use crate::routes::health_check::health_check;
use crate::routes::subscriptions::subscribe;

pub fn run(listener: TcpListener, connection: PgConnection) -> io::Result<Server> {
    let connection = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .route("/subscriptions", web::post().to(subscribe))
            .route("/health_check", web::get().to(health_check))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
