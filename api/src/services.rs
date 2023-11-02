use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn get_health() -> impl Responder {
    HttpResponse::Ok()
}
