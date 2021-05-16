use std::net::TcpListener;

use actix_web::{App, HttpServer, web};
use actix_web::dev::Server;
use sqlx::PgPool;

use crate::routes::{healthz, subscribe};

pub fn run(
  listener: TcpListener,
  db_pool: PgPool,
) -> Result<Server, std::io::Error> {
  let db_pool = web::Data::new(db_pool);
  let server = HttpServer::new(move || {
    App::new()
      .route("/healthz", web::get().to(healthz))
      .route("/subscriptions", web::post().to(subscribe))
      .app_data(db_pool.clone())
  })
    .listen(listener)?
    .run();
  Ok(server)
}
