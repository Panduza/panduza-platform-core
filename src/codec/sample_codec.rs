use crate::{Error, MessageCodec};
use bytes::Bytes;
use bytes::BytesMut;
use flatbuffers::FlatBufferBuilder;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Display;

use super::sample_generated::SampleArgs;
use super::sample_generated::SampleBuilder;
use super::sample_generated::Sample;

///
/// Codec for a simple Boolean
///
#[derive(Clone, PartialEq, Debug)]
pub struct SampleCodec {
    pub value: bytes::Bytes,

}



impl SampleCodec 
{

    pub fn from_values(value: &Vec<f32>) -> Self {

        let mut builder = flatbuffers::FlatBufferBuilder::new();
        
        // https://github.com/google/flatbuffers/blob/master/samples/sample_binary.rs

        let inventory = builder.create_vector(value);

        let orc = Sample::create(&mut builder, &SampleArgs{
            values: Some(inventory)
        });
      
        builder.finish(orc, None);

        // Here we copy into the buffer
        Self {
            value: Bytes::from(builder.finished_data().to_vec())
        }
    }
}


///
/// To ease display
///
impl Display for SampleCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", 5))
        // self.value))
    }
}


///
/// To apply all the required trait
///
impl MessageCodec for SampleCodec {
    ///
    /// Manage deserialization
    ///
    fn from_message_payload(data: &bytes::Bytes) -> Result<SampleCodec, Error> {
        // // Convert incoming bytes into a str
        // let data_as_string =
        //     String::from_utf8(data.to_vec()).map_err(|e| Error::DeserializeError(e.to_string()))?;

        // // Deserialize the string
        // let p: SampleCodec = serde_json::from_str(data_as_string.as_str()).map_err(|e| {
        //     Error::DeserializeError(format!("serde_json fail on : {}", e.to_string()))
        // })?;

        // // let bytes = Bytes::from(buffer);

        // let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(data.len());
        // builder.
        
        // Get access to the root:
        // let monster = flatbuffers::root::<Sample>(data).unwrap();
        // let ff = monster.values();

        
        // let bufff = builder.finished_data();

        // let bbb = Bytes::from(bufff);

        // Return
        Ok(Self {
            value: data.clone()
        })
    }

    ///
    ///
    ///
    fn into_message_payload(&self) -> Result<Vec<u8>, Error> {
        // let v = serde_json::to_string(self).map_err(|e| Error::SerializeFailure(e.to_string()))?;
        Ok(self.value.to_vec())
    }

    ///
    fn typee() -> String {
        "sample".to_string()
    }
}
