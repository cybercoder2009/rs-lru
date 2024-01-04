#[cfg(test)]
mod tests {
    
    use lru::Lru;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_read_then_delete() {
        let lru: Arc<Mutex<Lru<&str, &str>>> = Arc::new(Mutex::new(Lru::new(2)));
        let mut cache = lru.lock().unwrap();
        cache.put("key1", "value1", 60); // 60 seconds expiration
        drop(cache);

        let lru_clone = Arc::clone(&lru);
        thread::spawn(move || {
            let mut cache = lru_clone.lock().unwrap();
            assert_eq!(cache.get(&"key1"), Some("value1")); // Read and hence remove
            assert_eq!(cache.get(&"key1"), None); // Should be None as it is removed
        }).join().unwrap();
    }

    #[test]
    fn test_expired_entry() {
        let lru: Arc<Mutex<Lru<&str, &str>>> = Arc::new(Mutex::new(Lru::new(2)));
        let mut cache = lru.lock().unwrap();
        cache.put("key1", "value1", 1); // 1 second expiration
        drop(cache);

        let lru_clone = Arc::clone(&lru);
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(2)); // Wait for expiration
            let mut cache = lru_clone.lock().unwrap();
            assert_eq!(cache.get(&"key1"), None); // Should be None as it is expired
        }).join().unwrap();
    }

    #[test]
    fn test_put_at_full_capacity_with_no_expired_entry() {
        let lru: Arc<Mutex<Lru<&str, &str>>> = Arc::new(Mutex::new(Lru::new(1))); // Capacity of 1
        let mut cache = lru.lock().unwrap();
        cache.put("key1", "value1", 60); // 60 seconds expiration

        assert!(!cache.put("key2", "value2", 60)); // Should return false as no space and no expired entry
    }
}
