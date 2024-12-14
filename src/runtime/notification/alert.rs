use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertNotification {
    ///
    ///
    ///
    pub topic: String,

    ///
    ///
    ///
    pub message: String,
}

impl AlertNotification {
    pub fn new(topic: String, message: String) -> Self {
        Self {
            topic: topic,
            message: message,
        }
    }
}
