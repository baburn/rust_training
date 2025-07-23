use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct CacheValue {
    pub value: String,
    pub expiry: Instant,
}

pub struct MemoryCache {
    store: Arc<Mutex<HashMap<String, CacheValue>>>,
}

impl MemoryCache {
    pub fn new() -> Self {
        let store = Arc::new(Mutex::new(HashMap::<String, CacheValue>::new()));
        let cloned_store = Arc::clone(&store);

        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(2));
            let mut map = cloned_store.lock().unwrap();
            map.retain(|_, v| v.expiry > Instant::now());
        });

        MemoryCache { store }
    }

    pub fn insert(&self, key: &str, value: &str, ttl_secs: u64) {
        let mut map = self.store.lock().unwrap();
        map.insert(
            key.to_string(),
            CacheValue {
                value: value.to_string(),
                expiry: Instant::now() + Duration::from_secs(ttl_secs),
            },
        );
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let map = self.store.lock().unwrap();
        map.get(key).map(|v| v.value.clone())
    }
}

impl Drop for MemoryCache {
    fn drop(&mut self) {
        println!("🔻 Cache dropped. Cleaning up resources...");
    }
}
