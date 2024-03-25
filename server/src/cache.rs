use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use chrono::DateTime;

pub struct CacheItem {
    value: String,
    expire_datetime: chrono::DateTime<chrono::Utc>,
}

// Define the main cache structure
pub struct Cache {
    items: Mutex<HashMap<String, CacheItem>>,
}

// Implementation of the Cache
impl Cache {
    // Constructor for the Cache
    pub fn new() -> Arc<Self> {
        Arc::new(Cache {
            items: Mutex::new(HashMap::new()),
        })
    }

    // Function to add items to the cache
    pub fn set(&self, key: String, value: String, expire_datetime: chrono::DateTime<chrono::Utc>) {
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
    pub fn get(&self, key: &str) -> Option<String>
        //where
        //    T: Clone,
    {
        let items = self.items.lock().unwrap();
        items.get(key).and_then(|item| {
            if item.expire_datetime > DateTime::now() {
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