use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
///
///
///
struct EnumSi {
    unit: String,
    min: i32,
    max: i32,
}

impl EnumSi {
    pub fn new<N: Into<String>>(unit: N, min: i32, max: i32) -> Self {
        Self {
            unit: unit.into(),
            min,
            max,
        }
    }
}

impl Into<JsonValue> for EnumSi {
    fn into(self) -> JsonValue {
        serde_json::to_value(self).unwrap()
    }
}
