use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let addr = spawn_app();

    let client = reqwest::Client::new();
    let resp = client
        .get(format!("http://{}/health_check", addr))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(resp.status().is_success());
    assert_eq!(Some(0), resp.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind ramdom port");
    let port = listener.local_addr().unwrap().port();
    let server = wpr::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("127.0.0.1:{}", port)
}
