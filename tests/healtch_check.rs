use zero2prod::run;

#[tokio::test]
async fn health_check_works() {
    // arrange
    spawn_app();
    // we need to bring in reqwest to perform HTTP requests against our application
    let client = reqwest::Client::new();

    // act
    let response = client.get("http://127.0.0.1:8123/health_check").send().await.expect("Failed to execute the request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = run().expect("Faild to bind address.");
    let _ = tokio::spawn(server);
}


