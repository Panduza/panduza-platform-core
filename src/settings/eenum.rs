use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
///
///
///
struct EnumSettings {
    choices: Vec<String>,
}

impl Into<JsonValue> for EnumSettings {
    fn into(self) -> JsonValue {
        todo!()
    }
}
