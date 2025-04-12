use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use tfhe::{FheUint8, FheUint64};
use std::time::{Duration, Instant};

// Make the value type generic
#[derive(Clone)]
pub enum CacheValue {
    U8(FheUint8),
    U64(FheUint64),
}

struct CacheEntry {
    value: CacheValue,
    created_at: Instant,
    ttl: Option<Duration>,
}

pub struct Cache {
    store: Arc<RwLock<HashMap<[u8; 32], CacheEntry>>>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn insert_u8(&self, key: [u8; 32], value: FheUint8, ttl: Option<Duration>) {
        let mut store = self.store.write().await;
        store.insert(key, CacheEntry {
            value: CacheValue::U8(value),
            created_at: Instant::now(),
            ttl,
        });
    }

    pub async fn insert_u64(&self, key: [u8; 32], value: FheUint64, ttl: Option<Duration>) {
        let mut store = self.store.write().await;
        store.insert(key, CacheEntry {
            value: CacheValue::U64(value),
            created_at: Instant::now(),
            ttl,
        });
    }

    pub async fn get_u8(&self, key: &[u8; 32]) -> Option<FheUint8> {
        let store = self.store.read().await;
        store.get(key).and_then(|entry| {
            if let Some(ttl) = entry.ttl {
                if entry.created_at.elapsed() > ttl {
                    return None;
                }
            }
            match &entry.value {
                CacheValue::U8(v) => Some(v.clone()),
                _ => None,
            }
        })
    }

    pub async fn get_u64(&self, key: &[u8; 32]) -> Option<FheUint64> {
        let store = self.store.read().await;
        store.get(key).and_then(|entry| {
            if let Some(ttl) = entry.ttl {
                if entry.created_at.elapsed() > ttl {
                    return None;
                }
            }
            match &entry.value {
                CacheValue::U64(v) => Some(v.clone()),
                _ => None,
            }
        })
    }
}

// Rest of the traits remain similar but work with CacheValue enum
impl CacheManagement for Cache {
    async fn cleanup_expired(&self) {
        let mut store = self.store.write().await;
        store.retain(|_, entry| {
            if let Some(ttl) = entry.ttl {
                entry.created_at.elapsed() <= ttl
            } else {
                true
            }
        });
    }
    // ... other methods
}
