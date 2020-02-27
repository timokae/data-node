use crate::logger;
use crate::models;

use models::ForeignHash;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock};
use std::thread_local;

struct StorageInner {
    data_map: HashMap<u64, String>,
    foreign_map: HashMap<u64, String>, // Hash, IP
}

pub struct Storage {
    inner: RwLock<StorageInner>,
}

impl Storage {
    pub fn new() -> Arc<Storage> {
        Arc::new(Storage {
            inner: RwLock::new(StorageInner {
                data_map: HashMap::new(),
                foreign_map: HashMap::new(),
            }),
        })
    }

    pub fn current() -> Arc<Storage> {
        CURRENT_STORAGE.with(|s| s.clone())
    }

    pub fn insert(&self, data: String) -> u64 {
        let hash = self.calculate_hash(&data);
        self.inner
            .write()
            .unwrap()
            .data_map
            .insert(hash, data.clone());

        let msg = format!("Inserted {} with hash {}", data.clone(), hash);
        logger::log("Storage", &msg);
        hash
    }

    pub fn get(&self, hash: u64) -> Option<String> {
        match self.inner.read().unwrap().data_map.get(&hash) {
            Some(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn get_foreign(&self, hash: u64) -> Option<String> {
        match self.inner.read().unwrap().foreign_map.get(&hash) {
            Some(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn insert_foreign(&self, new_hashes: Vec<ForeignHash>) {
        {
            let foreign_map = &mut self.inner.write().unwrap().foreign_map;
            foreign_map.clear();
            for f_hash in new_hashes {
                foreign_map.insert(f_hash.hash.parse::<u64>().unwrap(), f_hash.addr);
            }
        }
        println!("{:?}", self.inner.read().unwrap().foreign_map);
    }

    pub fn hashes(&self) -> Vec<String> {
        self.inner
            .read()
            .unwrap()
            .data_map
            .keys()
            .map(|key| key.to_string().clone())
            .collect()
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
