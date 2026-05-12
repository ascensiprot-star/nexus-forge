use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::RwLock;

pub struct MetricsRegistry {
    counters: RwLock<HashMap<String, AtomicU64>>,
    gauges: RwLock<HashMap<String, AtomicU64>>,
}

impl MetricsRegistry {
    pub fn new() -> Self {
        Self {
            counters: RwLock::new(HashMap::new()),
            gauges: RwLock::new(HashMap::new()),
        }
    }

    pub fn increment(&self, name: &str) {
        let counters = self.counters.read().unwrap();
        if let Some(counter) = counters.get(name) {
            counter.fetch_add(1, Ordering::Relaxed);
        } else {
            drop(counters);
            let mut counters = self.counters.write().unwrap();
            counters
                .entry(name.to_string())
                .or_insert_with(|| AtomicU64::new(1));
        }
    }

    pub fn set_gauge(&self, name: &str, value: u64) {
        let gauges = self.gauges.read().unwrap();
        if let Some(gauge) = gauges.get(name) {
            gauge.store(value, Ordering::Relaxed);
        } else {
            drop(gauges);
            let mut gauges = self.gauges.write().unwrap();
            gauges
                .entry(name.to_string())
                .or_insert_with(|| AtomicU64::new(value));
        }
    }

    pub fn get_counter(&self, name: &str) -> u64 {
        self.counters
            .read()
            .unwrap()
            .get(name)
            .map(|c| c.load(Ordering::Relaxed))
            .unwrap_or(0)
    }

    pub fn get_gauge(&self, name: &str) -> u64 {
        self.gauges
            .read()
            .unwrap()
            .get(name)
            .map(|g| g.load(Ordering::Relaxed))
            .unwrap_or(0)
    }

    pub fn snapshot(&self) -> HashMap<String, u64> {
        let mut snapshot = HashMap::new();
        for (name, counter) in self.counters.read().unwrap().iter() {
            snapshot.insert(format!("counter.{name}"), counter.load(Ordering::Relaxed));
        }
        for (name, gauge) in self.gauges.read().unwrap().iter() {
            snapshot.insert(format!("gauge.{name}"), gauge.load(Ordering::Relaxed));
        }
        snapshot
    }
}

impl Default for MetricsRegistry {
    fn default() -> Self {
        Self::new()
    }
}
