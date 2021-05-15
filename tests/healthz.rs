use std::net::TcpListener;

#[actix_rt::test]
async fn health_check_works() {
  let address = spawn_app();
  spawn_app();
  let client = reqwest::Client::new();

  let response = client
    .get(&format!("{}/healthz", &address))
    .send()
    .await
    .expect("Failed to execute request.");

  assert!(response.status().is_success());
  assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
  let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to port");
  let port = listener.local_addr().unwrap().port();


  let server = zero2prod::run(listener).expect("Failed to bind address");
  // Launch the server as a background task
  // tokio::spawn returns a handle to the spawned future,
  // but we have no use for it here, hence the non-binding let
  let _ = tokio::spawn(server);
  format!("http://127.0.0.1:{}", port)
}
