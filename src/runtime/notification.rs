use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateChangedNotification {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementNotification {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Notification {
    StateChanged(StateChangedNotification),
    ElementCreated(ElementNotification),
    ElementDeleted(ElementNotification),
}
