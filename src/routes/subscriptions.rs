use actix_web::{HttpResponse, web};
use chrono::Utc;
use sqlx::PgPool;
use tracing_futures::Instrument;
use uuid::Uuid;

pub async fn subscribe(
  form: web::Form<FormData>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
  let correlation_id = Uuid::new_v4();

  let request_span = tracing::info_span!(
    "Adding a new subscriber",
    %correlation_id,
    email = %form.email,
    name = %form.name
  );

  let _request_span_guard = request_span.enter();

  tracing::info!(
    "correlation_id: {}. Adding '{}' '{}' as a new subscriber.",
    correlation_id,
    form.email,
    form.name
  );

  let query_span = tracing::info_span!("Saving new subscriber details in the database.");
  sqlx::query!(
    r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
    "#,
    Uuid::new_v4(),
    form.email,
    form.name,
    Utc::now()
  )
    // There is a bit of ceremony here to get our hands on a &PgConnection.
    // web::Data<Arc<PgConnection>> is equivalent to Arc<Arc<PgConnection>>
    // Therefore connection.get_ref() returns a &Arc<PgConnection>
    // which we can then deref to a &PgConnection.
    // We could have avoided the double Arc wrapping using .app_data()
    // instead of .data() in src/startup.rs - we'll get to it later!
    .execute(pool.get_ref())
    .instrument(query_span)
    .await
    .map_err(|error| {
      tracing::error!("correlation_id: {}. Failed to execute query: {:?}", correlation_id, error);
      HttpResponse::InternalServerError().finish()
    })?;

  Ok(HttpResponse::Ok().finish())
}

#[derive(serde::Deserialize)]
pub struct FormData {
  email: String,
  name: String,
}