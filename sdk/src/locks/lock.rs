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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_new_lock() {
        let lock = Lock::new();
        assert!(lock.items.lock().unwrap().is_empty());
    }

    #[test]
    fn test_acquire_release() {
        let lock = Lock::new();
        let key = "test_key";

        // Test acquiring the lock
        assert!(lock.try_acquire(key));
        assert!(lock.is_acquire(key));

        // Test releasing the lock
        assert!(lock.try_release(key));
        assert!(!lock.is_acquire(key));
    }

    #[test]
    fn test_double_acquire() {
        let lock = Lock::new();
        let key = "test_key";

        // Acquire the lock once
        assert!(lock.try_acquire(key));
        // Try to acquire again, should fail
        assert!(!lock.try_acquire(key));
    }

    #[test]
    fn test_release_nonexistent_key() {
        let lock = Lock::new();
        let key = "test_key";

        // Try to release a key that was never acquired
        assert!(!lock.try_release(key));
    }

    #[test]
    fn test_concurrent_access() {
        let lock = Arc::new(Lock::new());
        let lock_clone = Arc::clone(&lock);
        let key = "concurrent_key";

        let handle = thread::spawn(move || {
            lock_clone.try_acquire(key)
        });

        // Wait for the thread to finish
        let result = handle.join().unwrap();

        // Check results
        assert!(result);
        assert!(lock.is_acquire(key));

        // Release should succeed
        assert!(lock.try_release(key));
        assert!(!lock.is_acquire(key));
    }
}