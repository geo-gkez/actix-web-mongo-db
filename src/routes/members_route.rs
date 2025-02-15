use crate::models::members_model::Members;
use crate::services::db::Database;
use actix_web::web::{Data, Json};
use actix_web::{get, post, web, HttpResponse, Responder};

pub fn init_members_route(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/members")
            .service(get_member_by_email)
            .service(get_members)
            .service(create_member)
            .service(hello),
    );
}
#[get("/hello")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[get("")]
pub async fn get_members(db: Data<Database>) -> impl Responder {
    match db.get_members().await {
        Ok(members) => HttpResponse::Ok().json(members),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string()),
    }
}

#[get("/{email}")]
pub async fn get_member_by_email(db: Data<Database>, email: web::Path<String>) -> impl Responder {
    match db.get_member_by_email(email.into_inner()).await {
        Ok(member) => HttpResponse::Ok().json(member),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string()),
    }
}

#[post("")]
pub async fn create_member(db: Data<Database>, request: Json<Members>) -> HttpResponse {
    match db.create_member(request.into_inner()).await {
        Ok(_) => HttpResponse::Created().json("Member created"),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string()),
    }
}
