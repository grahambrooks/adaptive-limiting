use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ResponseTimeChangeRequest {
    pub response_time: u64,
}
