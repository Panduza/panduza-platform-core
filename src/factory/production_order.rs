use std::ffi::{CStr, CString};

use serde_json::json;

pub type DeviceSettings = serde_json::Value;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ProductionOrder {
    /// Name of the device to be produced
    pub name: String,

    /// Reference of driver device producer
    pub dref: String,

    ///
    pub settings: DeviceSettings,
}

impl ProductionOrder {
    /// Constructor
    ///
    pub fn new<A: Into<String>, B: Into<String>>(d_ref: A, d_name: B) -> ProductionOrder {
        ProductionOrder {
            name: d_name.into(),
            dref: d_ref.into(),
            settings: serde_json::Value::Null,
        }
    }

    /// From a json value
    ///
    pub fn from_json(value: &serde_json::Value) -> ProductionOrder {
        ProductionOrder {
            name: "test".to_string(),
            dref: "rtok".to_string(),
            settings: json!({}),
        }
    }

    pub fn dref(&self) -> &String {
        &self.dref
    }

    /// Converts the ProductionOrder into a C string
    ///
    /// Don't forget "".as_c_str().as_ptr()" to use it with the DLL interfaces
    ///
    pub fn to_c_string(&self) -> Result<CString, crate::Error> {
        let json_str =
            serde_json::to_string(self).expect("Failed to serialize ProductionOrder to JSON");
        CString::new(json_str)
            .map_err(|e| crate::Error::InternalLogic(format!("Failed to build CString ({:?})", e)))
    }

    // pub fn from_c_str_ptr(&mut self, c_str: *const i8) {}
    // /// Converts a C-style string pointer into a `ProductionOrder`
    pub fn from_c_str_ptr(c_str: *const i8) -> Result<Self, crate::Error> {
        //
        //
        if c_str.is_null() {
            return Err(crate::Error::InvalidArgument(
                "Null C string pointer".to_string(),
            ));
        }

        //
        //
        let c_str = unsafe { CStr::from_ptr(c_str) };
        let str = c_str
            .to_str()
            .map_err(|e| crate::Error::InvalidArgument(format!("Invalid C string: {:?}", e)))?;

        let json: serde_json::Value = serde_json::from_str(str)
            .map_err(|e| crate::Error::InvalidArgument(format!("Invalid JSON: {:?}", e)))?;

        let obj = serde_json::from_value(json).map_err(|e| {
            crate::Error::InvalidArgument(format!("Failed to deserialize JSON: {:?}", e))
        })?;

        Ok(obj)
    }
}
