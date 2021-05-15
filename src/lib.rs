use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use actix_web::dev::Server;
use std::net::TcpListener;

async fn healthz() -> impl Responder {
  HttpResponse::Ok()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
  let server = HttpServer::new(|| {
    App::new()
      .route("/healthz", web::get().to(healthz))
  })
    .listen(listener)?
    .run();
  Ok(server)
}