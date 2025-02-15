mod models;
mod routes;
mod services;

use crate::routes::members_route::init_members_route;
use actix_web::{web::Data, App, HttpServer};
use services::db::Database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init().await;
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .configure(init_members_route)
    })
    .bind(("localhost", 8080))
    .map_err(|e| e.to_string())
    .expect("Can not bind to port 8080")
    .run()
    .await
    .map_err(|e| e.into())
}
