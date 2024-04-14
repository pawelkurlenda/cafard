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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};
    use std::thread;
    use std::time::Duration as StdDuration;

    #[test]
    fn test_set_and_get() {
        let cache = Cache::new();
        let key = "test_key".to_string();
        let value = "test_value".to_string();
        let expire_time = Utc::now() + Duration::seconds(60);

        cache.set(key.clone(), value.clone(), expire_time);

        assert_eq!(cache.get(&key).unwrap(), value);
    }

    #[test]
    fn test_expiration() {
        let cache = Cache::new();
        let key = "expired_key".to_string();
        let value = "expired_value".to_string();
        let expire_time = Utc::now() - Duration::seconds(1);  // Past expiration

        cache.set(key.clone(), value.clone(), expire_time);
        thread::sleep(StdDuration::from_secs(2)); // Ensure the time has passed for expiration check

        assert!(cache.get(&key).is_none());
    }

    #[test]
    fn test_delete() {
        let cache = Cache::new();
        let key = "delete_key".to_string();
        let value = "delete_value".to_string();
        let expire_time = Utc::now() + Duration::seconds(60);

        cache.set(key.clone(), value.clone(), expire_time);
        assert!(cache.get(&key).is_some());

        cache.delete(&key);
        assert!(cache.get(&key).is_none());
    }

    #[test]
    fn test_non_existent_key() {
        let cache = Cache::new();
        let key = "non_existent_key";

        assert!(cache.get(key).is_none());
    }
}