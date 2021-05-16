use actix_web::{HttpResponse, Responder};

pub(crate) async fn healthz() -> impl Responder {
  HttpResponse::Ok()
}
