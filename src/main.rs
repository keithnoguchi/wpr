use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .route("/health-check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {name}")
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[cfg(test)]
mod tests {
    use super::health_check;

    #[tokio::test]
    async fn health_check_success() {
        let resp = health_check().await;
        assert!(resp.status().is_success());
    }
}
