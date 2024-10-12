use serde::{Deserialize, Serialize};

pub mod state;
pub mod structural;

pub use state::StateNotification;
pub use structural::StructuralNotification;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Notification {
    StateChanged(StateNotification),
    ElementCreated(StructuralNotification),
    ElementDeleted(StructuralNotification),
}

impl Notification {
    // pub fn new_state_changed_notification() -> Notification {}
}
