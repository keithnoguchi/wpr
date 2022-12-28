use std::net::TcpListener;

use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

use wpr::config::DatabaseSettings;

struct TestApp {
    address: String,
    db_pool: PgPool,
}

impl TestApp {
    async fn configure_database(config: &DatabaseSettings) -> PgPool {
        let mut connection = PgConnection::connect(&config.connection_string_without_db())
            .await
            .expect("Failed to connect to postgres");
        connection
            .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
            .await
            .expect("Failed to create database");

        let db_pool = PgPool::connect(&config.connection_string())
            .await
            .expect("Failed to connect to postgres");
        sqlx::migrate!("./migrations")
            .run(&db_pool)
            .await
            .expect("Failed to migrate the database");

        db_pool
    }

    async fn build() -> TestApp {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind ramdom port");
        let port = listener.local_addr().unwrap().port();

        let mut config = wpr::config::load().expect("Failed to load configuration");
        config.database.database_name = Uuid::new_v4().to_string();
        let db_pool = Self::configure_database(&config.database).await;

        let server = wpr::startup::run(listener, db_pool.clone()).expect("Failed to bind address");
        let _ = tokio::spawn(server);

        Self {
            address: format!("http://127.0.0.1:{}", port),
            db_pool,
        }
    }
}

#[tokio::test]
async fn health_check_works() {
    let test_app = TestApp::build().await;

    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/health_check", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(resp.status().is_success());
    assert_eq!(Some(0), resp.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let test_app = TestApp::build().await;
    let client = reqwest::Client::new();

    let body = "name=someone&email=user%40gmail.com";
    let resp = client
        .post(&format!("{}/subscriptions", &test_app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, resp.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "user@gmail.com");
    assert_eq!(saved.name, "someone");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let test_app = TestApp::build().await;
    let client = reqwest::Client::new();
    let test_cases = [
        ("name=test%20name", "missing the email"),
        ("email=test%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let resp = client
            .post(&format!("{}/subscriptions", &test_app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            resp.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message,
        );
    }
}
