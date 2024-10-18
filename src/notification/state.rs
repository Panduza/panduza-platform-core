use serde::{Deserialize, Serialize};

use crate::device::State;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateNotification {
    ///
    object: String,

    state: State,
}

impl StateNotification {
    pub fn new(name: String, state: State) -> Self {
        Self {
            object: name,
            state: state,
        }
    }
}
