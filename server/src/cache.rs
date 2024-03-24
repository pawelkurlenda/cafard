use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

pub struct CacheItem<T> {
    value: T,
    expire_datetime: SystemTime,
}

// Define the main cache structure
pub struct Cache<T> {
    items: Mutex<HashMap<String, CacheItem<T>>>,
}

// Implementation of the Cache
impl<T> Cache<T> {
    // Constructor for the Cache
    pub fn new() -> Arc<Self> {
        Arc::new(Cache {
            items: Mutex::new(HashMap::new()),
        })
    }

    // Function to add items to the cache
    pub fn set(&self, key: String, value: T, expire_datetime: SystemTime) {
        let mut items = self.items.lock().unwrap();
        items.insert(
            key,
            CacheItem {
                value,
                expire_datetime,
            },
        );
    }

    // Function to retrieve items from the cache
    pub fn get(&self, key: &str) -> Option<T>
        where
            T: Clone,
    {
        let items = self.items.lock().unwrap();
        items.get(key).and_then(|item| {
            if item.expire_datetime > SystemTime::now() {
                Some(item.value.clone())
            } else {
                None
            }
        })
    }
}

pub struct CacheModel {
    key: String,
    value: String,
    pub expiration_datetime: chrono::DateTime<chrono::Utc>
}

pub fn add_or_update() {

}

pub fn delete(key: &str) {

}