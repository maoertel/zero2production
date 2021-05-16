use std::net::TcpListener;

use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use actix_web::dev::Server;

async fn healthz() -> impl Responder {
  HttpResponse::Ok()
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
  HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct FormData {
  email: String,
  name: String,
}

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
