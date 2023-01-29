use zero2prod::run;

#[tokio::test]
async fn health_check_works() {
    // arrange
    let address = spawn_app();
    // we need to bring in reqwest to perform HTTP requests against our application
    let client = reqwest::Client::new();

    // act
    let response = client.get(&format!("{}/health_check", &address))
                         .send()
                         .await
                         .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port.");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Faild to bind address.");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{port}")
}
