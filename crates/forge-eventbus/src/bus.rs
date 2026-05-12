use std::sync::Arc;

use async_trait::async_trait;
use forge_core::error::ForgeResult;
use forge_core::event::ForgeEvent;
use forge_core::traits::{EventBus, EventSubscription};

use crate::inmemory::InMemoryEventBus;

pub enum EventBusImpl {
    InMemory(Arc<InMemoryEventBus>),
}

impl EventBusImpl {
    pub fn in_memory() -> Self {
        Self::InMemory(Arc::new(InMemoryEventBus::new()))
    }
}

#[async_trait]
impl EventBus for EventBusImpl {
    async fn publish(&self, subject: &str, event: ForgeEvent) -> ForgeResult<()> {
        match self {
            Self::InMemory(bus) => bus.publish(subject, event).await,
        }
    }

    async fn subscribe(&self, subject: &str) -> ForgeResult<Box<dyn EventSubscription>> {
        match self {
            Self::InMemory(bus) => bus.subscribe(subject).await,
        }
    }

    async fn request(
        &self,
        subject: &str,
        event: ForgeEvent,
        timeout_ms: u64,
    ) -> ForgeResult<ForgeEvent> {
        match self {
            Self::InMemory(bus) => bus.request(subject, event, timeout_ms).await,
        }
    }
}
