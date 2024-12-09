use super::Settings as UsbSettings;
use crate::protocol::AsciiCmdRespProtocol;
use crate::{format_driver_error, log_debug, log_trace, DriverLogger, Error};
use async_trait::async_trait;
use serial2_tokio::SerialPort;
use std::time::Duration;
use tokio::time::timeout;

///
///
pub struct Driver {
    ///
    /// To help data logging inside the driver
    ///
    logger: DriverLogger,
}

impl Driver {
    /// Create a new instance of the driver
    ///
    pub fn open(settings: &UsbSettings, eol: Vec<u8>) -> Result<Self, Error> {
        //
        // Prepare logger
        let logger = DriverLogger::new("usb", "tmc", "");
        // log_debug!(logger, "Opening serial driver {:?}...", &port_name);

        Ok(Self { logger })
    }
}
