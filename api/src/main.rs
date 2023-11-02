mod db;
mod services;

use crate::db::Database;
use actix_web::{middleware, web, App, HttpServer};
use std::env;

pub struct AppState {
    pub app_name: String,
    pub database: Database,
}

impl AppState {
    pub fn new() -> Self {
        let db = Database::new();

        AppState {
            app_name: "app".to_string(),
            database: db,
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState::new()))
            .wrap(middleware::Logger::default())
            .service(services::get_health)
            .service(services::post_health)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
