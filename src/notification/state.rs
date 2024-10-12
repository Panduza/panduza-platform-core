use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateNotification {}

impl StateNotification {
    pub fn new() -> Self {
        Self {}
    }
}
