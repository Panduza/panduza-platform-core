use super::Notification;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
///
/// Notification for attribute or class deletion
///
pub struct DeletionNotification {
    ///
    /// Only the topic matter for deletion
    ///
    pub topic: String,
}

impl DeletionNotification {
    ///
    /// Create a new instance
    ///
    pub fn new<A: Into<String>>(name: A) -> Self {
        Self { topic: name.into() }
    }
}

///
/// Implicit convertion
///
impl Into<Notification> for DeletionNotification {
    fn into(self) -> Notification {
        Notification::ElementDeleted(self)
    }
}
