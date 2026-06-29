use crate::events::user_registered::UserRegisteredEventBus;

#[derive(Clone)]
pub struct EventsBus {
  pub  user_registered_event_bus: UserRegisteredEventBus,
}

impl EventsBus {
    pub fn new(user_registered_event_bus: UserRegisteredEventBus) -> Self {
        Self {
            user_registered_event_bus,
        }
    }
}
