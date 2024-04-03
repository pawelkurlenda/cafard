use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};

#[derive(Clone, Debug)]
pub struct CacheItem {
    value: String,
    expire_datetime: DateTime<Utc>,
}

#[derive(Debug)]
pub struct Cache {
    items: Mutex<HashMap<String, CacheItem>>,
}

impl Cache {
    pub fn new() -> Arc<Self> {
        Arc::new(Cache {
            items: Mutex::new(HashMap::new()),
        })
    }

    pub fn set(&self, key: String, value: String, expire_datetime: DateTime<Utc>) {
        let mut items = self.items.lock().unwrap();
        items.insert(
            key,
            CacheItem {
                value,
                expire_datetime,
            },
        );
    }

    pub fn get(&self, key: &str) -> Option<String>
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