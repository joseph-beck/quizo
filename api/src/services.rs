use crate::AppState;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct HealthInfo {
    post: String,
}

#[get("/")]
pub async fn get_health(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    let db = &data.database;

    match db.health_check() {
        Ok(_) => HttpResponse::Ok().body(format!("Passed Get Health Check {}", app_name)),
        Err(_) => HttpResponse::BadRequest().body(format!("Failed Get Health Check, {}", app_name)),
    }
}

#[post("/{post}")]
pub async fn post_health(data: web::Data<AppState>, info: web::Path<HealthInfo>) -> impl Responder {
    let app_name = &data.app_name;
    let post = &info.post;
    HttpResponse::Ok().body(format!("Passed Post Health Check, {}, {}", app_name, post))
}
