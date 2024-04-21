use std::sync::{Arc, Mutex};

use cafard::cache::Cache;
use cafard::geospatial::Geospatial;
use cafard::lock::Lock;

pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
}

#[derive(Clone)]
pub struct CacheState {
    pub cache: Arc<Cache>,
}

#[derive(Clone)]
pub struct LockState {
    pub lock: Arc<Lock>,
}

#[derive(Clone)]
pub struct GeospatialState {
    pub lock: Arc<Geospatial>,
}
