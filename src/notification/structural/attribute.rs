use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttributeMode {
    #[serde(rename = "RO")]
    AttOnly,
    #[serde(rename = "WO")]
    CmdOnly,
    #[serde(rename = "RW")]
    Bidir,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeNotification {
    name: String,
    typee: String,
    mode: AttributeMode,
    settings: Option<JsonValue>,
}

impl AttributeNotification {
    ///
    ///
    ///
    pub fn new<N: Into<String>, T: Into<String>>(
        name: N,
        typee: T,
        mode: AttributeMode,
        settings: Option<JsonValue>,
    ) -> Self {
        Self {
            name: name.into(),
            typee: typee.into(),
            mode,
            settings: settings,
        }
    }

    ///
    /// Topic getter
    ///
    pub fn topic(&self) -> String {
        self.name.clone()
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
    pub fn settings(&self) -> &Option<JsonValue> {
        &self.settings
    }
}
