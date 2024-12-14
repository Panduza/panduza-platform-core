pub mod alert;
pub mod creation;
pub mod enablement;
pub mod group;
pub mod state;

pub use alert::AlertNotification;
pub use creation::CreationNotification;
pub use enablement::EnablementNotification;
pub use state::StateNotification;

use crate::instance::State;
use creation::{attribute::AttributeMode, AttributeNotification, InterfaceNotification};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Available Runtime Notification Types
///
pub enum Notification {
    /// There is a warning message coming from the instance
    ///
    Alert(AlertNotification),

    /// An instance state has changed
    ///
    State(StateNotification),

    /// An attribute or a class has been created
    ///
    /// Deletion does not exist, once created only the instance destruction
    /// can erase the attribute or the class. Choose Enable/Disable instead.
    ///
    Creation(CreationNotification),

    /// An attribute or a class has been enabled or disabled
    ///
    Enablement(EnablementNotification),
}

impl Notification {
    ///
    /// TODO => make it deprecated
    ///
    pub fn new_alert_notification(topic: String, message: String) -> Notification {
        Notification::Alert(AlertNotification::new(topic, message))
    }

    ///
    /// TODO => make it deprecated
    ///
    pub fn new_interface_element_created_notification<N: Into<String>>(
        topic: N,
        tags: Vec<String>,
    ) -> Notification {
        Notification::Creation(CreationNotification::Interface(InterfaceNotification::new(
            topic, tags,
        )))
    }

    ///
    /// TODO => make it deprecated
    ///
    pub fn new_attribute_element_created_notification<N: Into<String>, T: Into<String>>(
        topic: N,
        typee: T,
        mode: AttributeMode,
        info: Option<String>,
        settings: Option<serde_json::Value>,
    ) -> Notification {
        Notification::Creation(CreationNotification::Attribute(AttributeNotification::new(
            topic, typee, mode, info, settings,
        )))
    }
}
