use std::sync::{Arc, Mutex};
use crate::cache::Cache;

pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
}

#[derive(Clone, Debug)]
pub struct CacheState {
    pub cache: Arc<Cache>,
}
