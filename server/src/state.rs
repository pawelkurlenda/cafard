use std::sync::Mutex;

pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
}

#[derive(Clone, Debug)]
pub struct CacheState {
    pub key: String,
    pub value: String,
    pub expiration_datetime: chrono::DateTime<chrono::Utc>
}
