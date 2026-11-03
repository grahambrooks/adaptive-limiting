use crate::models::ResponseTimeChangeRequest;
use actix_web::{HttpResponse, web};
use log::info;

pub async fn hello(data: web::Data<crate::AppState>) -> HttpResponse {
    let response_time = data.response_time.lock().unwrap();
    info!("/ requested delay: {}ms", *response_time);
    std::thread::sleep(std::time::Duration::from_millis(*response_time));
    HttpResponse::Ok().json(serde_json::json!({ "message": "hello" }))
}

pub async fn set_response_time(
    data: web::Data<crate::AppState>,
    req: web::Json<ResponseTimeChangeRequest>,
) -> HttpResponse {
    let mut response_time = data.response_time.lock().unwrap();
    *response_time = req.response_time;
    HttpResponse::Ok().finish()
}
