use serde::{Deserialize, Serialize};
pub mod alert;
pub mod creation;
pub mod deletion;
pub mod group;
pub mod state;
use crate::instance::State;
pub use alert::AlertNotification;
pub use creation::CreationNotification;
use creation::{attribute::AttributeMode, AttributeNotification, InterfaceNotification};
pub use deletion::DeletionNotification;
pub use state::StateNotification;

#[derive(Debug, Clone, Serialize, Deserialize)]
///
/// Available Notification Types
///
pub enum Notification {
    ///
    /// There is a warning message coming from the instance
    ///
    Alert(AlertNotification),

    ///
    /// An instance state has changed
    ///
    StateChanged(StateNotification),

    ///
    /// An attribute or a class has been created
    ///
    ElementCreated(CreationNotification),

    ///
    /// An attribute or a class has been deleted
    ///
    ElementDeleted(DeletionNotification),
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
        Notification::ElementCreated(CreationNotification::Interface(InterfaceNotification::new(
            topic, tags,
        )))
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
        Notification::ElementCreated(CreationNotification::Attribute(AttributeNotification::new(
            topic, typee, mode, info, settings,
        )))
    }
}
