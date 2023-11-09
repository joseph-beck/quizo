use crate::models::User;
use crate::AppState;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::fmt::format;

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

#[get("/api/v1/user")]
pub async fn list_user(data: web::Data<AppState>) -> impl Responder {
    let db = &data.database;
    let app_name = &data.app_name;

    let user_result = db.user_list(0);
    match user_result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::BadRequest().body(format!("failure in {} to list users", app_name)),
    }
}

#[get("/api/v1/user/{uuid}")]
pub async fn get_user(data: web::Data<AppState>, info: web::Path<UserInfo>) -> impl Responder {
    let app_name = &data.app_name;
    let db = &data.database;
    let uuid = &info.uuid;

    let user_exists_result = db.user_exists(uuid);
    match user_exists_result {
        Ok(user_exists) => {
            if !user_exists {
                return HttpResponse::NotFound().body(format!("could not find user: {}", uuid));
            }
        }
        Err(_) => {
            return HttpResponse::BadRequest()
                .body(format!("failure in {} to get user: {}", app_name, uuid))
        }
    }

    let user_result = db.user_get(uuid);
    match user_result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::BadRequest()
            .body(format!("failure in {} to get user: {}", app_name, uuid)),
    }
}
