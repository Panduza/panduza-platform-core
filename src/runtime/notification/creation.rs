pub mod attribute;
mod interface;

pub use attribute::AttributeNotification;
pub use interface::InterfaceNotification;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CreationNotification {
    Attribute(AttributeNotification),
    Interface(InterfaceNotification),
}

impl CreationNotification {
    ///
    ///
    ///
    pub fn topic(&self) -> String {
        match self {
            CreationNotification::Attribute(attribute_notification) => {
                attribute_notification.topic()
            }
            CreationNotification::Interface(interface_notification) => {
                interface_notification.topic()
            }
        }
    }
}
