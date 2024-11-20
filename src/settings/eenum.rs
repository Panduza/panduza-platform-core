use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
///
///
///
pub struct EnumSettings {
    choices: Vec<String>,
}

impl From<Vec<String>> for EnumSettings {
    fn from(values: Vec<String>) -> Self {
        Self { choices: values }
    }
}

impl From<Vec<&str>> for EnumSettings {
    fn from(values: Vec<&str>) -> Self {
        Self {
            choices: values.iter().map(|&s| s.into()).collect(),
        }
    }
}

impl Into<JsonValue> for EnumSettings {
    fn into(self) -> JsonValue {
        serde_json::to_value(self.choices).unwrap()
    }
}
