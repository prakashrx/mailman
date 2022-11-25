use mailman::run;


#[tokio::test]
async fn health_check_works() {
    spawn_app();

    let client = reqwest::Client::new();
    let response = client
    .get("http://localhost:8080/health")
    .send()
    .await
    .expect("Failed to reach health check endpoint");

    assert!(response.status().is_success());
    assert_eq!(0, response.content_length().unwrap());
}

fn spawn_app() {
    let server = run().expect("Failed to launch app");
    let _ = tokio::spawn(server);
}