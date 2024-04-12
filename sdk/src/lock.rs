use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::cache::Cache;

#[derive(Debug)]
pub struct Lock {
    items: Mutex<HashMap<String, bool>>,
}

impl Lock {
    pub fn new() -> Arc<Self> {
        Arc::new(Lock {
            items: Mutex::new(HashMap::new()),
        })
    }

    pub fn acquire(&self, lock_key: &str) -> bool {
        let mut items = self.items.lock().unwrap();

        match items.get(&lock_key.into_inner()) {
            Some(true) => false,
            _ => {
                items.insert(lock_key.into_inner(), true);
                true
            }
        }
    }

    pub fn release(&self, lock_key: &str) -> bool{
        let mut items = self.items.lock().unwrap();

        match items.get(&lock_key.into_inner()) {
            Some(true) => {
                items.insert(lock_key.into_inner(), false);
                true
            },
            _ => false,
        }
    }

    pub fn get_status(&self, lock_key: &str) -> bool {
        let mut items = self.items.lock().unwrap();

        items.get(&lock_key.into_inner())
    }
}