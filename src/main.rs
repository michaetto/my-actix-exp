use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use std::io;

mod http_api;
use crate::http_api::data::health::{HealthStatusResponse, HealthStatus};

async fn handler() -> impl Responder {
    // serde_json::to_string(&HealthStatusResponse{status: HealthStatus::Healthy})
    HttpResponse::Ok().json(HealthStatusResponse{status: HealthStatus::Healthy})
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/", web::get().to(handler))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}