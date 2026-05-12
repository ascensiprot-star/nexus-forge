use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use forge_core::error::{ForgeError, ForgeResult};
use forge_core::event::ForgeEvent;
use forge_core::traits::{EventBus, EventSubscription};
use tokio::sync::{broadcast, RwLock};

use crate::subscription::BroadcastSubscription;

const CHANNEL_CAPACITY: usize = 1024;

pub struct InMemoryEventBus {
    channels: RwLock<HashMap<String, broadcast::Sender<ForgeEvent>>>,
}

impl InMemoryEventBus {
    pub fn new() -> Self {
        Self {
            channels: RwLock::new(HashMap::new()),
        }
    }

    async fn get_or_create_channel(&self, subject: &str) -> broadcast::Sender<ForgeEvent> {
        {
            let channels = self.channels.read().await;
            if let Some(sender) = channels.get(subject) {
                return sender.clone();
            }
        }

        let mut channels = self.channels.write().await;
        let (tx, _) = broadcast::channel(CHANNEL_CAPACITY);
        channels.entry(subject.to_string()).or_insert(tx).clone()
    }

    fn matches_subject(pattern: &str, subject: &str) -> bool {
        if pattern == subject {
            return true;
        }
        if pattern.ends_with(".>") {
            let prefix = &pattern[..pattern.len() - 2];
            return subject.starts_with(prefix);
        }
        if pattern.contains('*') {
            let parts: Vec<&str> = pattern.split('.').collect();
            let subject_parts: Vec<&str> = subject.split('.').collect();
            if parts.len() != subject_parts.len() {
                return false;
            }
            return parts
                .iter()
                .zip(subject_parts.iter())
                .all(|(p, s)| *p == "*" || p == s);
        }
        false
    }
}

impl Default for InMemoryEventBus {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EventBus for InMemoryEventBus {
    async fn publish(&self, subject: &str, event: ForgeEvent) -> ForgeResult<()> {
        let channels = self.channels.read().await;

        for (pattern, sender) in channels.iter() {
            if Self::matches_subject(pattern, subject) || pattern == subject {
                let _ = sender.send(event.clone());
            }
        }

        if !channels.contains_key(subject) {
            drop(channels);
            let sender = self.get_or_create_channel(subject).await;
            let _ = sender.send(event);
        }

        Ok(())
    }

    async fn subscribe(&self, subject: &str) -> ForgeResult<Box<dyn EventSubscription>> {
        let sender = self.get_or_create_channel(subject).await;
        let receiver = sender.subscribe();
        Ok(Box::new(BroadcastSubscription::new(receiver)))
    }

    async fn request(
        &self,
        subject: &str,
        event: ForgeEvent,
        timeout_ms: u64,
    ) -> ForgeResult<ForgeEvent> {
        let reply_subject = format!("{subject}._reply.{}", event.id);
        let mut sub = self.subscribe(&reply_subject).await?;

        self.publish(subject, event).await?;

        let timeout = tokio::time::Duration::from_millis(timeout_ms);
        match tokio::time::timeout(timeout, sub.next()).await {
            Ok(Ok(Some(reply))) => Ok(reply),
            Ok(Ok(None)) => Err(ForgeError::Timeout {
                operation: format!("request to {subject}"),
                timeout_ms,
            }),
            Ok(Err(e)) => Err(e),
            Err(_) => Err(ForgeError::Timeout {
                operation: format!("request to {subject}"),
                timeout_ms,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use forge_core::event::EventMetadata;
    use forge_core::types::ProjectId;

    fn test_event(event_type: &str) -> ForgeEvent {
        ForgeEvent::new(
            event_type,
            "test",
            serde_json::json!({"test": true}),
            EventMetadata {
                project_id: ProjectId::new(),
                user_id: None,
                agent_id: None,
                task_id: None,
            },
        )
    }

    #[tokio::test]
    async fn test_publish_subscribe() {
        let bus = InMemoryEventBus::new();
        let mut sub = bus.subscribe("test.topic").await.unwrap();

        let event = test_event("test.event");
        bus.publish("test.topic", event.clone()).await.unwrap();

        let received =
            tokio::time::timeout(std::time::Duration::from_millis(100), sub.next()).await;

        assert!(received.is_ok());
    }

    #[test]
    fn test_subject_matching() {
        assert!(InMemoryEventBus::matches_subject(
            "forge.task.>",
            "forge.task.created"
        ));
        assert!(InMemoryEventBus::matches_subject(
            "forge.task.>",
            "forge.task.completed"
        ));
        assert!(!InMemoryEventBus::matches_subject(
            "forge.task.>",
            "forge.agent.created"
        ));
        assert!(InMemoryEventBus::matches_subject(
            "forge.*.created",
            "forge.task.created"
        ));
        assert!(!InMemoryEventBus::matches_subject(
            "forge.*.created",
            "forge.task.completed"
        ));
    }
}
