use crate::Error;
use serde_json::json;

/// Key for the usb serial in the json settings
static USB_SERIAL_KEY: &str = "usb_serial";

/// Usb settings for devices
#[derive(Debug)]
pub struct Settings {
    /// VID
    pub vendor: Option<u16>,

    /// PID
    pub model: Option<u16>,

    /// Serial String
    pub serial: Option<String>,
}

impl Settings {
    /// Creates a new Settings instance
    ///
    pub fn new() -> Settings {
        Settings {
            vendor: None,
            model: None,
            serial: None,
        }
    }

    /// Set the vendor
    ///
    pub fn set_vendor(mut self, vendor: u16) -> Self {
        self.vendor = Some(vendor);
        self
    }

    /// Set the model
    ///
    pub fn set_model(mut self, model: u16) -> Self {
        self.model = Some(model);
        self
    }

    /// Extracts the serial port name from the json settings
    /// This function fails if the settings is not present or ill-formed
    ///
    pub fn set_serial_from_json_settings(
        mut self,
        settings: &serde_json::Value,
    ) -> Result<Self, Error> {
        self.serial = Some(
            settings
                .get(USB_SERIAL_KEY)
                .ok_or(Error::BadSettings(format!(
                    "Unable to get \"{}\"",
                    USB_SERIAL_KEY
                )))?
                .as_str()
                .ok_or(Error::BadSettings(format!(
                    "\"{}\" not a string",
                    USB_SERIAL_KEY
                )))?
                .to_string(),
        );
        Ok(self)
    }

    /// Like `set_serial_from_json_settings` but with a default value in case
    /// of error on settings extraction
    ///
    pub fn set_serial_from_json_settings_or(
        mut self,
        settings: &serde_json::Value,
        default: &str,
    ) -> Self {
        let default_as_value = json!(default);
        self.serial = Some(
            settings
                .get(USB_SERIAL_KEY)
                .unwrap_or_else(|| &default_as_value)
                .as_str()
                .unwrap_or_else(|| default)
                .to_string(),
        );
        self
    }

    /// Like `set_serial_from_json_settings` but with a default value in case
    /// of error on settings extraction
    ///
    pub fn optional_set_serial_from_json_settings(mut self, settings: &serde_json::Value) -> Self {
        self.serial = match settings.get(USB_SERIAL_KEY) {
            Some(serial) => match serial.as_str() {
                Some(s) => Some(s.to_string()),
                None => None,
            },
            None => None,
        };
        self
    }
}

impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let vendor = self.vendor.unwrap_or(0);
        let model = self.model.unwrap_or(0);
        write!(
            f,
            "Settings {{ vendor: {:#02x}, model: {:#02x}, serial: {:?} }}",
            vendor, model, self.serial
        )
    }
}
