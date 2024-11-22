use serde::{Deserialize, Serialize};

pub mod alert;
pub mod group;
pub mod state;
pub mod structural;

pub use alert::AlertNotification;
pub use state::StateNotification;
pub use structural::StructuralNotification;
use structural::{attribute::AttributeMode, AttributeNotification, InterfaceNotification};

use crate::instance::State;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Notification {
    Alert(AlertNotification),
    StateChanged(StateNotification),
    ElementCreated(StructuralNotification),
    ElementDeleted(StructuralNotification),
}

impl Notification {
    ///
    ///
    ///
    pub fn new_state_changed_notification(name: String, state: State) -> Notification {
        Notification::StateChanged(StateNotification::new(name, state))
    }

    ///
    ///
    ///
    pub fn new_alert_notification(topic: String, message: String) -> Notification {
        Notification::Alert(AlertNotification::new(topic, message))
    }

    ///
    ///
    ///
    pub fn new_interface_element_created_notification<N: Into<String>>(
        topic: N,
        tags: Vec<String>,
    ) -> Notification {
        Notification::ElementCreated(StructuralNotification::Interface(
            InterfaceNotification::new(topic, tags),
        ))
    }

    ///
    ///
    ///
    pub fn new_attribute_element_created_notification<N: Into<String>, T: Into<String>>(
        topic: N,
        typee: T,
        mode: AttributeMode,
        info: Option<String>,
        settings: Option<serde_json::Value>,
    ) -> Notification {
        Notification::ElementCreated(StructuralNotification::Attribute(
            AttributeNotification::new(topic, typee, mode, info, settings),
        ))
    }
}
