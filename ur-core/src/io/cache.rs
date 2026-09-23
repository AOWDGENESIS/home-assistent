//! Bounded in-memory sector cache.

use std::collections::{HashMap, VecDeque};

/// A byte-bounded FIFO/LRU-like cache for sector reads.
#[derive(Debug)]
pub struct SectorCache {
    entries: HashMap<u64, Vec<u8>>,
    order: VecDeque<u64>,
    max_size_bytes: usize,
    current_size_bytes: usize,
    hits: u64,
    lookups: u64,
}

impl SectorCache {
    /// Create a cache with the given capacity in mebibytes.
    pub fn new(max_size_mb: usize) -> Self {
        Self {
            entries: HashMap::new(),
            order: VecDeque::new(),
            max_size_bytes: max_size_mb.saturating_mul(1024 * 1024),
            current_size_bytes: 0,
            hits: 0,
            lookups: 0,
        }
    }

    /// Return a cached sector and refresh its recency.
    pub fn get(&mut self, sector: u64) -> Option<Vec<u8>> {
        self.lookups += 1;
        let value = self.entries.get(&sector).cloned();
        if value.is_some() {
            self.hits += 1;
            self.touch(sector);
        }
        value
    }

    /// Insert or replace a sector, evicting old entries as needed.
    pub fn insert(&mut self, sector: u64, data: Vec<u8>) {
        if data.len() > self.max_size_bytes {
            return;
        }
        if let Some(previous) = self.entries.remove(&sector) {
            self.current_size_bytes = self.current_size_bytes.saturating_sub(previous.len());
            self.order.retain(|key| *key != sector);
        }
        while self.current_size_bytes + data.len() > self.max_size_bytes {
            let Some(oldest) = self.order.pop_front() else {
                break;
            };
            if let Some(value) = self.entries.remove(&oldest) {
                self.current_size_bytes = self.current_size_bytes.saturating_sub(value.len());
            }
        }
        self.current_size_bytes += data.len();
        self.entries.insert(sector, data);
        self.order.push_back(sector);
    }

    /// Cache hit ratio, or zero if no lookup has occurred.
    pub fn hit_rate(&self) -> f32 {
        if self.lookups == 0 {
            0.0
        } else {
            self.hits as f32 / self.lookups as f32
        }
    }

    fn touch(&mut self, sector: u64) {
        self.order.retain(|key| *key != sector);
        self.order.push_back(sector);
    }
}

#[cfg(test)]
mod tests {
    use super::SectorCache;

    #[test]
    fn evicts_oldest_entry_when_full() {
        let mut cache = SectorCache::new(1);
        cache.insert(1, vec![1; 700_000]);
        cache.insert(2, vec![2; 700_000]);
        assert!(cache.get(1).is_none());
        assert_eq!(cache.get(2), Some(vec![2; 700_000]));
    }
}
