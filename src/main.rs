use std::net::TcpListener;

use sqlx::PgPool;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, Registry};

use zero2prod::config::get_configuration;
use zero2prod::startup::run;

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

  LogTracer::init().expect("Failed to set logger");
  let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
  let formatting_layer = BunyanFormattingLayer::new(
    "zero2prod".into(),
    std::io::stdout,
  );

  let subscriber = Registry::default()
    .with(env_filter)
    .with(JsonStorageLayer)
    .with(formatting_layer);

  set_global_default(subscriber).expect("Failed to set subscriber.");

  run(listener, connection_pool)?.await
}
