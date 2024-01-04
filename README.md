# Time-based LRU Cache

* It is not thread safe. Arc<Mutex<Lru>> is required for multi-thread.
* ```get``` removes entry.