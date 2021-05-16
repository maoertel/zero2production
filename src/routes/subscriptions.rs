use actix_web::{HttpResponse, web};

pub(crate) async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
  HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
pub struct FormData {
  email: String,
  name: String,
}