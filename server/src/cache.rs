use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};

#[derive(Clone, Debug)]
pub struct CacheItem {
    value: String,
    expire_datetime: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
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
        let mut items = self.items.lock().unwrap();
        if let Some(item) = items.get(key) {
            if item.expire_datetime > Utc::now() {
                Some(item.value.clone())
            } else {
                // If the item is expired, remove it from the cache
                items.remove(key);
                None
            }
        } else {
            None
        }
    }

    pub fn delete(&self, key: &str) {
        let mut items = self.items.lock().unwrap();
        items.remove(key);
    }
}

/*pub struct CacheModel {
    key: String,
    value: String,
    pub expiration_datetime: chrono::DateTime<chrono::Utc>
}*/