use std::io;
use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;

use crate::routes::health_check::health_check;
use crate::routes::subscriptions::subscribe;

pub fn run(listener: TcpListener, pg_pool: PgPool) -> io::Result<Server> {
    let pg_pool = web::Data::new(pg_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/subscriptions", web::post().to(subscribe))
            .route("/health_check", web::get().to(health_check))
            .app_data(pg_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
