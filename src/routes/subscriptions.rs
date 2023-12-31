use uuid::Uuid;
use chrono::Utc;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
        _form: web::Form<FormData>,
        _pool: web::Data<PgPool>) -> HttpResponse {
    
    // Let's generate a random unique identifier
    let request_id = Uuid::new_v4();
    
    tracing::info!("request_id {} - Adding '{}' '{}' as a new subscriber.", 
            request_id,
            _form.email, _form.name);
    
    tracing::info!("request_id {} - Saving new subscriber details in the database.",
            request_id);

    match sqlx::query!(
        r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
            "#,
        request_id,
        _form.email,
        _form.name,
        Utc::now()
    )
    .execute(_pool.as_ref())
    .await
    {
        Ok(_) => {
            tracing::info!("request_id {} -  New subscribe details have been saved.", request_id);
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            tracing::error!("request_id {}, - Failed to execute query: {:?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}