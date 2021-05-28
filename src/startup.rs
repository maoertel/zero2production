use std::net::TcpListener;

use actix_web::{App, HttpServer, web};
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

use crate::routes::{healthz, subscribe};

pub fn run(
  listener: TcpListener,
  db_pool: PgPool,
) -> Result<Server, std::io::Error> {
  let db_pool = web::Data::new(db_pool);
  let server = HttpServer::new(move || {
    App::new()
      .wrap(TracingLogger::default())
      .route("/healthz", web::get().to(healthz))
      .route("/subscriptions", web::post().to(subscribe))
      .app_data(db_pool.clone())
  })
    .listen(listener)?
    .run();
  Ok(server)
}
