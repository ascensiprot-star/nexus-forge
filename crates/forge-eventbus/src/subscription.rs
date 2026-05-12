use async_trait::async_trait;
use forge_core::error::ForgeResult;
use forge_core::event::ForgeEvent;
use forge_core::traits::EventSubscription;
use tokio::sync::broadcast;

pub struct BroadcastSubscription {
    receiver: broadcast::Receiver<ForgeEvent>,
}

impl BroadcastSubscription {
    pub fn new(receiver: broadcast::Receiver<ForgeEvent>) -> Self {
        Self { receiver }
    }
}

#[async_trait]
impl EventSubscription for BroadcastSubscription {
    async fn next(&mut self) -> ForgeResult<Option<ForgeEvent>> {
        match self.receiver.recv().await {
            Ok(event) => Ok(Some(event)),
            Err(broadcast::error::RecvError::Closed) => Ok(None),
            Err(broadcast::error::RecvError::Lagged(n)) => {
                tracing::warn!("subscription lagged by {n} messages");
                match self.receiver.recv().await {
                    Ok(event) => Ok(Some(event)),
                    Err(_) => Ok(None),
                }
            }
        }
    }

    async fn unsubscribe(self: Box<Self>) -> ForgeResult<()> {
        Ok(())
    }
}
