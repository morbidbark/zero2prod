use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    Ok(HttpServer::new(|| {
        App::new()
            .route("/subscriptions", web::post().to(subscribe))
            .route("/health_check", web::get().to(health_check))
    })
    .listen(listener)?
    .run())
}
