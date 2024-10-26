use serde::Deserialize;

#[derive(Deserialize)]
pub struct ResponseTimeChangeRequest {
    pub response_time: u64,
}
