use serde::{Deserialize, Serialize};

use crate::device::State;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateNotification {
    ///
    pub topic: String,

    pub state: State,
}

impl StateNotification {
    pub fn new(name: String, state: State) -> Self {
        Self {
            topic: name,
            state: state,
        }
    }
}
