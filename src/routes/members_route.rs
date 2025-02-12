use actix_web::{post, HttpResponse};
use actix_web::web::{Data, Json};
use crate::models::members_model::Members;
use crate::services::db::Database;

#[post("/members")]
pub async fn create_member(db: Data<Database>, request: Json<Members>) -> HttpResponse {
    db.create_member(request.into_inner())
        .await
        .map(|_| HttpResponse::Created().finish())
        .unwrap_or_else(|_| HttpResponse::InternalServerError().finish())
}
