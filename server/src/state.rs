use std::sync::{Arc, Mutex};

use cafard::cache::Cache;

pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
}

#[derive(Clone)]
pub struct CacheState {
    pub cache: Arc<Cache>,
}
