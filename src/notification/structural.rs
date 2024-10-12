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
