mod models;
mod services;
mod routes;

use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
use services::db::Database;
use crate::routes::members_route::create_member;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init().await;
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(hello)
            .service(create_member)
    }
    )
        .bind(("localhost", 8080))?
        .run()
        .await
}
