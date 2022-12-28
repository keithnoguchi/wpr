use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;

pub(crate) async fn subscribe(
    form: web::Form<FormData>,
    pg_pool: web::Data<PgPool>,
) -> HttpResponse {
    info!(
        "Adding '{}' '{}' as a new subscriber",
        form.email, form.name,
    );
    info!("Saving new subscriber details in the database");
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
    .map(|_| {
        info!("New subscriber details have been saved");
        HttpResponse::Ok().finish()
    })
    .unwrap_or_else(|e| {
        error!("Failed to execute query: {e}");
        HttpResponse::InternalServerError().finish()
    })
}

#[derive(serde::Deserialize)]
pub(crate) struct FormData {
    email: String,
    name: String,
}
