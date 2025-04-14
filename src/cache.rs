use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use tfhe::{FheUint8, FheUint64};

// Make the value type generic
#[derive(Clone)]
pub enum CacheValue {
    U8(FheUint8),
    U64(FheUint64),
}

pub struct Cache {
    store: Arc<RwLock<HashMap<[u8; 32], CacheValue>>>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn insert_u64(&self, key: [u8; 32], value: FheUint64) {
        let mut store = self.store.write().await;
        store.insert(key, CacheValue::U64(value));
    }

    pub async fn get_u64(&self, key: &[u8; 32]) -> Option<FheUint64> {
        let store = self.store.read().await;
        match store.get(key) {
            Some(CacheValue::U64(v)) => Some(v.clone()),
            _ => None,
        }
    }
}