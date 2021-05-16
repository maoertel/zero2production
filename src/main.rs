use std::net::TcpListener;

use zero2prod::config::get_configuration;
use zero2prod::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let configuration = get_configuration().expect("Failed to read configuration.");
  let host = configuration.server.host;
  let port = configuration.server.port;
  let address = format!("{}:{}", host, port);

  let listener = TcpListener::bind(address)?;

  run(listener)?.await
}
