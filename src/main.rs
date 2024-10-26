use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

mod handlers;
mod models;
mod lib;

struct AppState {
    response_time: Mutex<u64>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        response_time: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/hello", web::get().to(handlers::hello))
            .route("/set_response_time", web::post().to(handlers::set_response_time))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
