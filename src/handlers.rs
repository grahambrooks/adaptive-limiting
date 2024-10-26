use crate::models::ResponseTimeChangeRequest;
use actix_web::{web, HttpResponse};
use log::info;
use std::sync::Mutex;

pub async fn hello(data: web::Data<crate::AppState>) -> HttpResponse {

    let response_time = data.response_time.lock().unwrap();
    info!("/ requested delay: {}ms", *response_time);
    std::thread::sleep(std::time::Duration::from_millis(*response_time));
    HttpResponse::Ok().json(serde_json::json!({ "message": "hello" }))
}

pub async fn set_response_time(data: web::Data<crate::AppState>, req: web::Json<ResponseTimeChangeRequest>) -> HttpResponse {
    let mut response_time = data.response_time.lock().unwrap();
    *response_time = req.response_time;
    HttpResponse::Ok().finish()
}

pub struct AppState {
    pub response_time: Mutex<u64>,
}

pub fn update_response_time(state: &AppState, new_time: u64) {
    let mut response_time = state.response_time.lock().unwrap();
    *response_time = new_time;
}
