use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
///
/// Type of a Prop (match json types)
///
pub enum PropType {
    Bool,
    Number,
    String,
    Array,
    #[default]
    Object,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
///
///
///
pub struct Prop {
    ///
    ///
    ///
    pub description: String,

    ///
    ///
    ///
    pub r#type: PropType,

    ///
    ///
    ///
    pub default: JsonValue,
}

impl Prop {
    ///
    ///
    ///
    pub fn new<T: Into<String>>(description: T, r#type: PropType, default: JsonValue) -> Self {
        Self {
            description: description.into(),
            r#type: r#type,
            default: default,
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
///
/// Represent a group of Prop
///
pub struct Props {
    ///
    ///
    ///
    entries: HashMap<String, Prop>,
}

impl Props {
    ///
    ///
    ///
    pub fn add_entry<A: Into<String>, T: Into<String>>(
        &mut self,
        name: A,
        description: T,
        r#type: PropType,
        default: JsonValue,
    ) {
        self.entries
            .insert(name.into(), Prop::new(description, r#type, default));
    }
}
