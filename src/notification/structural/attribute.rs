use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttributeMode {
    AttOnly,
    CmdOnly,
    Bidir,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeNotification {
    name: String,
    typee: String,
    mode: AttributeMode,
}

impl AttributeNotification {
    ///
    ///
    ///
    pub fn new<N: Into<String>, T: Into<String>>(name: N, typee: T, mode: AttributeMode) -> Self {
        Self {
            name: name.into(),
            typee: typee.into(),
            mode,
        }
    }

    ///
    /// Topic getter
    ///
    pub fn topic(&self) -> String {
        self.name.clone()
    }

    pub fn into_json_value(&self) -> serde_json::Value {
        json!({
            // "name": self.name,
            "type": self.typee,
            "mode": self.mode
        })
    }

    ///
    pub fn name(&self) -> &String {
        &self.name
    }
    ///
    pub fn typee(&self) -> &String {
        &self.typee
    }
    pub fn mode(&self) -> &AttributeMode {
        &self.mode
    }

    // ///
    // /// Attribute does not hold any elements
    // ///
    // pub fn is_element_exist(&self, layers: Vec<String>) -> Result<bool, Error> {
    //     Ok(false)
    // }
}
