use actix_web::{web, App, HttpServer, Responder};
use log::{info, LevelFilter};
use serde::{Deserialize, Serialize};
use simple_logger::SimpleLogger;
use std::env;
use std::sync::Mutex;
mod handlers;
mod models;

struct AppState {
    response_time: Mutex<u64>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();
    let app_state = web::Data::new(AppState {
        response_time: Mutex::new(100),
    });

    let listen_port = env::var("LISTEN_PORT").unwrap_or_else(|_| "8080".to_string());

    info!("Starting server on http://{}:{}", hostname::get().unwrap().to_string_lossy(), listen_port);


    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/", web::get().to(handlers::hello))
            .route("/response/time", web::post().to(handlers::set_response_time))
    })
        .bind(format!("127.0.0.1:{}", listen_port))?  // Bind
        .run()
        .await
}
