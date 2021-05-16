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

#[actix_rt::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
  // Given
  let app_address = spawn_app();
  let client = reqwest::Client::new();
  let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

  // When
  let response = client
    .post(&format!("{}/subscriptions", &app_address))
    .header("Content-Type", "application/x-www-form-urlencoded")
    .body(body)
    .send()
    .await
    .expect("Failed to execute request.");

  // Then
  assert_eq!(200, response.status().as_u16());
}

#[actix_rt::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
  // Given
  let app_address = spawn_app();
  let client = reqwest::Client::new();
  let test_cases = vec![
    ("name=le%20guin", "missing the email"),
    ("email=ursula_le_guin%40gmail.com", "missing the name"),
    ("", "missing both name and email")
  ];
  for (invalid_body, error_message) in test_cases {
    // When
    let response = client
      .post(&format!("{}/subscriptions", &app_address))
      .header("Content-Type", "application/x-www-form-urlencoded")
      .body(invalid_body)
      .send()
      .await
      .expect("Failed to execute request.");
    // Then
    assert_eq!(
      400,
      response.status().as_u16(),
      "The API did not fail with 400 Bad Request when the payload was {}.",
      error_message
    );
  }
}
