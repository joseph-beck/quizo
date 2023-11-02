use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use crate::db::Database;

pub struct AppState {
    app_name: String,
    db: Database,
}

impl AppState {
    pub fn new() -> Self {
        let db = Database::new();

        AppState {
            app_name: "app".to_string(),
            db,
        }
    }
}

#[get("/")]
pub async fn get_health(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    let db = &data.db;

    match db.health_check() {
        Ok(_) => HttpResponse::Ok().body("Passed Health Check"),
        Err(_) => HttpResponse::BadRequest().body("Failed Health Check"),
    }
}

