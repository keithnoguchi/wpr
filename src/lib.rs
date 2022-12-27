use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::io;

pub fn run() -> io::Result<Server> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind("127.0.0.1:8000")?
        .run();

    Ok(server)
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}