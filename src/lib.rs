use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

fn timestamp() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

pub struct Entry<V> {
    value: V,
    expire: u64, // expire timestamp (secs)
}

impl<V> Entry<V> {
    fn is_expired(&self) -> bool {
        timestamp() >= self.expire
    }
}

pub struct Lru<K, V> {
    capacity: usize,
    map: HashMap<K,Entry<V>>,
}

impl<K, V> Lru<K, V>
where
    K: std::hash::Hash + std::cmp::Eq + std::clone::Clone,
    V: std::clone::Clone,
{
    pub fn new(capacity: usize) -> Self {
        Lru {
            capacity,
            map: HashMap::new(),
        }
    }

    pub fn get(&mut self, k: &K) -> Option<V> {
        match self.map.remove(k) {
            Some(opt) => {
                match opt.is_expired() {
                    false => Some(opt.value.clone()),
                    true => None,
                }
            },
            None => None,
        }
    }

    /**
     * capacity => full => remove expired entry => true  => return true
     *                                          => false => return false
     * capacity => has space                             => return true   
     */
    pub fn put(&mut self, k: K, v: V, expire_secs: u64) -> bool {
        if self.map.len() == self.capacity {
            let mut expired = None;
            for (k, v) in &self.map {
                if v.is_expired() {
                    expired = Some(k.clone());
                    break;
                }
            }
            if let Some(k) = expired {
                self.map.remove(&k);
                self.map.insert(k, Entry { value: v, expire: timestamp() + expire_secs });
                true
            } else {
                false
            }
        } else {
            self.map.insert(k, Entry { value: v, expire: timestamp() + expire_secs });
            true
        }
    }
}