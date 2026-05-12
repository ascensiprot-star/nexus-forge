use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryEvent {
    pub event_type: String,
    pub source: String,
    pub timestamp: DateTime<Utc>,
    pub data: serde_json::Value,
}

pub struct TelemetryCollector {
    events: std::sync::Mutex<Vec<TelemetryEvent>>,
    max_buffer: usize,
}

impl TelemetryCollector {
    pub fn new(max_buffer: usize) -> Self {
        Self {
            events: std::sync::Mutex::new(Vec::new()),
            max_buffer,
        }
    }

    pub fn record(&self, event_type: &str, source: &str, data: serde_json::Value) {
        let event = TelemetryEvent {
            event_type: event_type.to_string(),
            source: source.to_string(),
            timestamp: Utc::now(),
            data,
        };

        let mut events = self.events.lock().unwrap();
        if events.len() >= self.max_buffer {
            events.remove(0);
        }
        events.push(event);
    }

    pub fn drain(&self) -> Vec<TelemetryEvent> {
        let mut events = self.events.lock().unwrap();
        std::mem::take(&mut *events)
    }

    pub fn event_count(&self) -> usize {
        self.events.lock().unwrap().len()
    }
}

impl Default for TelemetryCollector {
    fn default() -> Self {
        Self::new(10_000)
    }
}
