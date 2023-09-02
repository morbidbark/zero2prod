use std::net::TcpListener;
#[tokio::test]
async fn health_check() {
    let addr = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{}/health_check", addr))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");
    let addr = listener.local_addr().unwrap().to_string();
    let server = zero2prod::run(listener).expect("Failed to start server");
    let _ = tokio::spawn(server);
    addr
}
