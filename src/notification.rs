use serde::{Deserialize, Serialize};

pub mod state;
pub mod structural;

pub use state::StateNotification;
use structural::InterfaceNotification;
pub use structural::StructuralNotification;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Notification {
    StateChanged(StateNotification),
    ElementCreated(StructuralNotification),
    ElementDeleted(StructuralNotification),
}

impl Notification {
    pub fn new_state_changed_notification() -> Notification {
        Notification::StateChanged(StateNotification::new())
    }

    pub fn new_interface_element_created_notification<N: Into<String>>(
        topic: N,
        tags: Vec<String>,
    ) -> Notification {
        Notification::ElementCreated(StructuralNotification::Interface(
            InterfaceNotification::new(topic, tags),
        ))
    }
}
