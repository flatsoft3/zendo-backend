use tokio::sync::broadcast;
use crate::models::user::User;

#[derive(Clone, Debug)]
pub struct UserRegisteredEvent{
    pub user: User
}

#[derive(Clone)]
pub struct UserRegisteredEventBus{
    tx: broadcast::Sender<UserRegisteredEvent>
}

impl UserRegisteredEventBus {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(64);
        Self {tx}
    }

    pub fn publish (&self, event: UserRegisteredEvent){
        let _ = self.tx.send(event);
    }

    pub fn subscribe(&self) -> broadcast::Receiver<UserRegisteredEvent> {
        self.tx.subscribe()
    }
}