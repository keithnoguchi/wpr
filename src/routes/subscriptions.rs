use actix_web::{web, HttpResponse};

pub(crate) async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
pub(crate) struct FormData {
    email: String,
    name: String,
}
