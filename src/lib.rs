use std::sync::Mutex;

pub struct AppState {
    pub response_time: Mutex<u64>,
}

pub fn update_response_time(state: &AppState, new_time: u64) {
    let mut response_time = state.response_time.lock().unwrap();
    *response_time = new_time;
}
