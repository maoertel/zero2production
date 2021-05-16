use std::net::TcpListener;

use sqlx::PgPool;

use zero2prod::config::get_configuration;
use zero2prod::startup::run;

#[actix_rt::test]
async fn health_check_works() {
  let app = spawn_app().await;
  let client = reqwest::Client::new();

  let response = client
    .get(&format!("{}/healthz", &app.address))
    .send()
    .await
    .expect("Failed to execute request.");

  assert!(response.status().is_success());
  assert_eq!(Some(0), response.content_length());
}

pub struct TestApp {
  pub address: String,
  pub db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
  let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to port");
  let port = listener.local_addr().unwrap().port();
  let address = format!("http://127.0.0.1:{}", port);

  let config = get_configuration().expect("Failed to read configuration.");

  let connection_pool = PgPool::connect(&config.database.connection_string())
    .await
    .expect("Failed to connect ot Postgres.");

  let server = run(listener, connection_pool.clone()).expect("Failed to bind address");
  // Launch the server as a background task
  // tokio::spawn returns a handle to the spawned future,
  // but we have no use for it here, hence the non-binding let
  let _ = tokio::spawn(server);

  TestApp { address, db_pool: connection_pool }
}

#[actix_rt::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
  // Given
  let app = spawn_app().await;
  let client = reqwest::Client::new();
  let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

  // When
  let response = client
    .post(&format!("{}/subscriptions", &app.address))
    .header("Content-Type", "application/x-www-form-urlencoded")
    .body(body)
    .send()
    .await
    .expect("Failed to execute request.");

  // Then
  assert_eq!(200, response.status().as_u16());

  let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
    .fetch_one(&app.db_pool)
    .await
    .expect("Failed to fetch saved subscription.");

  assert_eq!(saved.email, "ursula_le_guin@gmail.com");
  assert_eq!(saved.name, "le guin");
}

#[actix_rt::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
  // Given
  let app = spawn_app().await;
  let client = reqwest::Client::new();
  let test_cases = vec![
    ("name=le%20guin", "missing the email"),
    ("email=ursula_le_guin%40gmail.com", "missing the name"),
    ("", "missing both name and email")
  ];
  for (invalid_body, error_message) in test_cases {
    // When
    let response = client
      .post(&format!("{}/subscriptions", &app.address))
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
