use crate::models::members_model::Members;
use crate::services::db::Database;
use actix_web::web::{Data, Json};
use actix_web::{get, post, web, HttpResponse, Responder};

pub fn init_members_route(config: &mut web::ServiceConfig) {
    config.service(web::scope("/members").service(create_member).service(hello));
}
#[get("")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[post("")]
pub async fn create_member(db: Data<Database>, request: Json<Members>) -> HttpResponse {
    match db.create_member(request.into_inner()).await {
        Ok(_) => HttpResponse::Created().json("Member created"),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string()),
    }
}
