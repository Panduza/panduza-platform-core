use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{Error, MessageCodec};

#[derive(Clone, PartialEq, Debug)]
pub struct SiCodec {
    value: serde_json::Value,
}

///
/// Allow implicit convertion
///
impl From<f32> for SiCodec {
    fn from(value: f32) -> SiCodec {
        SiCodec {
            value: serde_json::json!(value),
        }
    }
}

///
/// Allow implicit convertion
///
impl From<u32> for SiCodec {
    fn from(value: u32) -> SiCodec {
        SiCodec {
            value: serde_json::json!(value),
        }
    }
}

///
/// Allow implicit convertion
///
impl Into<SiCodec> for u64 {
    fn into(self) -> SiCodec {
        return SiCodec {
            value: serde_json::json!(self),
        };
    }
}

///
/// Allow implicit convertion
///
impl Into<SiCodec> for u16 {
    fn into(self) -> SiCodec {
        return SiCodec {
            value: serde_json::json!(self),
        };
    }
}

///
/// Allow implicit convertion
///
impl Into<SiCodec> for i32 {
    fn into(self) -> SiCodec {
        return SiCodec {
            value: serde_json::json!(self),
        };
    }
}

///
/// Do not use derive because we do not want { "value": true }
/// But only true or false on the payload
///
impl Serialize for SiCodec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.value.serialize(serializer)
    }
}

///
/// See Serialize
///
impl<'de> Deserialize<'de> for SiCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        Ok(SiCodec { value })
    }
}

impl MessageCodec for SiCodec {
    ///
    ///
    ///
    fn from_message_payload(data: &bytes::Bytes) -> Result<Self, Error> {
        let p: Self =
            serde_json::from_str(String::from_utf8(data.to_vec()).unwrap().as_str()).unwrap();
        Ok(p)
    }
    ///
    ///
    ///
    fn into_message_payload(&self) -> Result<Vec<u8>, Error> {
        let v = serde_json::to_string(self).map_err(|e| Error::SerializeFailure(e.to_string()))?;
        Ok(v.into_bytes())
    }

    ///
    fn typee() -> String {
        "si".to_string()
    }
}
