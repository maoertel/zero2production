use std::net::TcpListener;

use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use actix_web::dev::Server;

use routes::health_check::healthz;
use routes::subscriptions::subscribe;

use crate::routes;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
  let server = HttpServer::new(|| {
    App::new()
      .route("/healthz", web::get().to(healthz))
      .route("/subscriptions", web::post().to(subscribe))
  })
    .listen(listener)?
    .run();
  Ok(server)
}
