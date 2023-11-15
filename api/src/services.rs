use crate::models::User;
use crate::AppState;
use actix_web::{delete, error, get, http::Error, patch, post, put, web, HttpResponse, Responder};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::fmt::format;

const MAX_SIZE: usize = 262_144;

#[derive(Deserialize)]
struct HealthInfo {
    post: String,
}

#[derive(Deserialize)]
struct UserInfo {
    uuid: String,
}

#[get("/api/v1/health")]
pub async fn get_health(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    let db = &data.database;

    match db.health_check() {
        Ok(_) => HttpResponse::Ok().body(format!("Passed Get Health Check {}", app_name)),
        Err(_) => HttpResponse::BadRequest().body(format!("Failed Get Health Check, {}", app_name)),
    }
}

#[post("/api/v1/health/{post}")]
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

#[post("/api/v1/user")]
pub async fn post_user(data: web::Data<AppState>, mut payload: web::Payload) -> impl Responder {
    let app_name = &data.app_name;
    let db = &data.database;

    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // body is loaded, now we can deserialize serde-json
    let user = serde_json::from_slice::<User>(&body)?;
    let add_result = db.user_add(user.clone());
    match add_result {
        Ok(_) => Ok(HttpResponse::Ok().body(format!("{} Added", user.uuid))),
        Err(_) => Ok(HttpResponse::BadRequest()
            .body(format!("Failed to add {} in app {}", user.uuid, app_name))),
    }
}

#[put("/api/v1/user")]
pub async fn put_user(data: web::Data<AppState>, mut payload: web::Payload) -> impl Responder {
    let app_name = &data.app_name;
    let db = &data.database;

    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // body is loaded, now we can deserialize serde-json
    let user = serde_json::from_slice::<User>(&body)?;
    let exists_result = db.user_exists(&user.uuid);
    // this is such a mess
    match exists_result {
        Ok(exists) => match exists {
            true => match db.user_update(user.clone()) {
                Ok(_) => Ok(Ok::<HttpResponse, Error>(HttpResponse::Ok().finish())),
                Err(_) => Ok(Ok::<HttpResponse, Error>(
                    HttpResponse::BadRequest().finish(),
                )),
            },
            false => match db.user_add(user.clone()) {
                Ok(_) => Ok(Ok::<HttpResponse, Error>(HttpResponse::Ok().finish())),
                Err(_) => Ok(Ok::<HttpResponse, Error>(
                    HttpResponse::BadRequest().finish(),
                )),
            },
        },
        Err(_) => Ok(Ok(HttpResponse::BadRequest().body(format!(
            "failed to update {} in app {}",
            &user.uuid, app_name
        )))),
    }
}

#[patch("/api/v1/user")]
pub async fn patch_user(data: web::Data<AppState>, mut payload: web::Payload) -> impl Responder {
    let app_name = &data.app_name;
    let db = &data.database;

    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // body is loaded, now we can deserialize serde-json
    let user = serde_json::from_slice::<User>(&body)?;
    let patch_result = db.user_update(user.clone());
    match patch_result {
        Ok(_) => Ok(HttpResponse::Ok().body(format!("{} Updated", &user.uuid))),
        Err(_) => Ok(HttpResponse::BadRequest().body(format!(
            "failed to update {} in app {}",
            &user.uuid, app_name
        ))),
    }
}

#[delete("/api/v1/user/{uuid}")]
pub async fn delete_user(data: web::Data<AppState>, info: web::Path<UserInfo>) -> impl Responder {
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

    let delete_result = db.user_delete(uuid);
    match delete_result {
        Ok(_) => HttpResponse::Ok().body(format!("{} Deleted", uuid)),
        Err(_) => HttpResponse::BadRequest()
            .body(format!("failed to delete {} in app {}", uuid, app_name)),
    }
}

#[cfg(test)]
mod tests {
    use actix_web::{test, App};

    use super::*;

    #[actix_web::test]
    async fn test_get_health() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState::new()))
                .service(get_health),
        )
        .await;
        let req = test::TestRequest::get().uri("/api/v1/health").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(
            resp.status().is_success(),
            "status code {:?}",
            resp.status()
        );
    }

    #[actix_web::test]
    async fn test_post_health() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState::new()))
                .service(post_health),
        )
        .await;
        let req = test::TestRequest::post().uri("/api/v1/health/hello").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(
            resp.status().is_success(),
            "status code {:?}, {:?}",
            resp.status(), resp.response()
        );
    }
}
