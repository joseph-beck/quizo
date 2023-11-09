use crate::AppState;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::fmt::format;
use crate::models::User;

#[derive(Deserialize)]
struct HealthInfo {
    post: String,
}

#[derive(Deserialize)]
struct UserInfo {
    uuid: String,
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

#[get("/api/v1/user/{uuid}")]
pub async fn get_user(data: web::Data<AppState>, info: web::Path<UserInfo>) -> impl Responder {
    let app_name = &data.app_name;
    let db = &data.database;
    let uuid = &info.uuid;

    let user = User{
        uuid: uuid.to_string(),
        username: "david".to_string(),
        password: "ab671?@23hash".to_string(),
        email: "dave@gmail.com".to_string(),
    };

    HttpResponse::Ok().json(user)
}
