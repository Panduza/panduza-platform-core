use crate::Error;
use nusb::DeviceInfo;
use serde_json::json;

/// Key for the usb serial in the json settings
static USB_SERIAL_KEY: &str = "usb_serial";

static USB_VID_KEY: &str = "usb_vid";
static USB_PID_KEY: &str = "usb_pid";

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

    ///
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

    /// Look into a json settings object and try to extract usb configuration
    ///
    ///
    pub fn optional_set_serial_from_json_settings(mut self, settings: &serde_json::Value) -> Self {
        if let Some(vendor) = settings.get(USB_VID_KEY) {
            if let Some(s) = vendor.as_u64() {
                self.vendor = Some(s as u16);
            }
        }
        if let Some(model) = settings.get(USB_PID_KEY) {
            if let Some(s) = model.as_u64() {
                self.model = Some(s as u16);
            }
        }
        self.serial = match settings.get(USB_SERIAL_KEY) {
            Some(serial) => match serial.as_str() {
                Some(s) => Some(s.to_string()),
                None => None,
            },
            None => None,
        };
        self
    }

    /// Look into a json settings object and try to extract usb configuration
    ///
    pub fn try_extract_from_json_settings(mut self, json_settings: &serde_json::Value) -> Self {
        //
        // Try to extract vid
        if let Some(vendor) = json_settings.get(USB_VID_KEY) {
            if let Some(s) = vendor.as_u64() {
                self.vendor = Some(s as u16);
            }
        }
        //
        // Try to extract pid
        if let Some(model) = json_settings.get(USB_PID_KEY) {
            if let Some(s) = model.as_u64() {
                self.model = Some(s as u16);
            }
        }
        //
        // Try to extract serial number
        self.serial = match json_settings.get(USB_SERIAL_KEY) {
            Some(serial) => match serial.as_str() {
                Some(s) => Some(s.to_string()),
                None => None,
            },
            None => None,
        };
        self
    }

    ///
    ///
    ///
    pub fn find_usb_device(&self) -> Option<DeviceInfo> {
        //
        //
        let mut found_device = None;
        //
        //
        for dev in nusb::list_devices().unwrap() {
            //
            //
            if let Some(v_vid) = self.vendor {
                if dev.vendor_id() != v_vid {
                    continue;
                }
            }
            //
            //
            if let Some(v_pid) = self.model {
                if dev.product_id() != v_pid {
                    continue;
                }
            }

            found_device = Some(dev);
        }
        found_device
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
