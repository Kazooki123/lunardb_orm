use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

pub struct CacheEntry<T> {
    data: T,
    expiry: Instant,
}

pub struct Cache<T> {
    store: RwLock<HashMap<String, CacheEntry<T>>>,
    ttl: Duration,
}

impl<T: Clone> Cache<T> {
    pub fn new(ttl_seconds: u64) -> Self {
        Self {
            store: RwLock::new(HashMap::new()),
            ttl: Duration::from_secs(ttl_seconds),
        }
    }

    pub async fn set(&self, key: &str, value: T) {
        let mut store = self.store.write().await;
        store.insert(
            key.to_string(),
            CacheEntry {
                data: value,
                expiry: Instant::now() + self.ttl,
            },
        );
    }

    pub async fn get(&self, key: &str) -> Option<T> {
        let store = self.store.read().await;
        if let Some(entry) = store.get(key) {
            if Instant::now() < entry.expiry {
                return Some(entry.data.clone());
            }
        }
        None
    }

    pub async fn delete(&self, key: &str) {
        let mut store = self.store.write().await;
        store.remove(key);
    }

    pub async fn clear(&self) {
        let mut store = self.store.write().await;
        store.clear();
    }

    pub async fn cleanup_expired(&self) {
        let mut store = self.store.write().await;
        store.retain(|_, entry| Instant::now() < entry.expiry);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_cache_operations() {
        let cache: Cache<String> = Cache::new(1); // 1 second TTL

        // Test set and get
        cache.set("key1", "value1".to_string()).await;
        assert_eq!(cache.get("key1").await, Some("value1".to_string()));

        // Test expiration
        sleep(Duration::from_secs(2)).await;
        assert_eq!(cache.get("key1").await, None);

        // Test delete
        cache.set("key2", "value2".to_string()).await;
        cache.delete("key2").await;
        assert_eq!(cache.get("key2").await, None);

        // Test clear
        cache.set("key3", "value3".to_string()).await;
        cache.clear().await;
        assert_eq!(cache.get("key3").await, None);
    }
}
