use crate::logger;

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock};
use std::thread_local;

struct StorageInner {
    data_map: HashMap<u64, String>
}

pub struct Storage {
    inner: RwLock<StorageInner>
}

impl Storage {
    pub fn new() -> Arc<Storage> {
        Arc::new(Storage {
            inner: RwLock::new(StorageInner {
                data_map: HashMap::new()
            })
        })
    }

    pub fn current() -> Arc<Storage> {
        CURRENT_STORAGE.with( |s| s.clone() )
    }

    pub fn insert(&self, data: String) -> u64 {
        let hash = self.calculate_hash(&data);
        self.inner.write().unwrap().data_map.insert(hash, data.clone());

        let msg = format!("Inserted {} with hash {}", data.clone(), hash);
        logger::log("Storage", &msg);

        hash
    }

    pub fn get(&self, hash: u64) -> Option<String> {
        match self.inner.read().unwrap().data_map.get(&hash) {
            Some(value) => {
                Some(value.clone())
            },
            _ => {
                None
            }
        }
    }

    fn calculate_hash<T: Hash>(&self, t: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        t.hash(&mut hasher);
        hasher.finish()
    }
}

thread_local! {
    static CURRENT_STORAGE: Arc<Storage> = Storage::new();
}