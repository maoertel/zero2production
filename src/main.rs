use std::net::TcpListener;

use sqlx::PgPool;

use zero2prod::config::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let configuration = get_configuration().expect("Failed to read configuration.");
  let host = configuration.server.host;
  let port = configuration.server.port;

  let address = format!("{}:{}", host, port);
  let listener = TcpListener::bind(address)?;

  let connection_pool = PgPool::connect(&configuration.database.connection_string())
    .await
    .expect("Failed to connect ot Postgres.");

  let subscriber = get_subscriber("zero2prod".into(), "info".into());
  init_subscriber(subscriber);

  run(listener, connection_pool)?.await
}
