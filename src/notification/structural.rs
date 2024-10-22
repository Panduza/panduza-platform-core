pub mod attribute;
mod interface;

pub use attribute::AttributeNotification;
pub use interface::InterfaceNotification;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StructuralNotification {
    Attribute(AttributeNotification),
    Interface(InterfaceNotification),
}

impl StructuralNotification {
    ///
    ///
    ///
    pub fn topic(&self) -> String {
        match self {
            StructuralNotification::Attribute(attribute_notification) => {
                attribute_notification.topic()
            }
            StructuralNotification::Interface(interface_notification) => {
                interface_notification.topic()
            }
        }
    }
}
