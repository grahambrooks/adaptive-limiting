use actix_web::{web, HttpResponse};
use serde::Deserialize;
use std::sync::Mutex;

use crate::models::ResponseTimeChangeRequest;
use crate::lib::update_response_time;

pub async fn hello(data: web::Data<crate::AppState>) -> HttpResponse {
    let response_time = data.response_time.lock().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(*response_time));
    HttpResponse::Ok().json("hello")
}

pub async fn set_response_time(
    data: web::Data<crate::AppState>,
    req: web::Json<ResponseTimeChangeRequest>,
) -> HttpResponse {
    let mut response_time = data.response_time.lock().unwrap();
    *response_time = req.response_time;
    HttpResponse::Ok().finish()
}
