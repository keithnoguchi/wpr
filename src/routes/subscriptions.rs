use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

pub(crate) async fn subscribe(
    form: web::Form<FormData>,
    pg_pool: web::Data<PgPool>,
) -> HttpResponse {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pg_pool.get_ref())
    .await
    .map(|_| HttpResponse::Ok().finish())
    .unwrap_or_else(|e| {
        println!("Failed to execute query: {e}");
        HttpResponse::InternalServerError().finish()
    })
}

#[derive(serde::Deserialize)]
pub(crate) struct FormData {
    email: String,
    name: String,
}
