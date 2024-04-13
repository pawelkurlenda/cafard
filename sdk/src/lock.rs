use std::collections::HashSet;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Lock {
    items: Mutex<HashSet<String>>,
}

impl Lock {
    pub fn new() -> Arc<Self> {
        Arc::new(Lock {
            items: Mutex::new(HashSet::new()),
        })
    }

    pub fn try_acquire(&self, lock_key: &str) -> bool {
        let mut items = self.items.lock().unwrap();

        if items.contains(lock_key) {
            false
        } else {
            items.insert(lock_key.to_owned());
            true
        }
    }

    pub fn try_release(&self, lock_key: &str) -> bool{
        let mut items = self.items.lock().unwrap();

        if items.contains(lock_key) {
            items.remove(lock_key);
            true
        } else {
            false
        }
    }

    pub fn is_acquire(&self, lock_key: &str) -> bool {
        let items = self.items.lock().unwrap();

        items.contains(lock_key)
    }
}