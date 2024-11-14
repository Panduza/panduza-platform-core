use std::time::Duration;

use super::generic::Driver as GenericDriver;
use super::Settings as SerialSettings;
use crate::Error;
use tokio::time::timeout;

use crate::format_driver_error;
use crate::log_debug;
use crate::log_info;

use super::SerialDriver;
use async_trait::async_trait;

///
///
pub struct Driver {
    ///
    ///
    ///
    base: GenericDriver,

    eol: Vec<u8>,
}

impl Driver {
    /// Create a new instance of the driver
    ///
    pub fn open(settings: &SerialSettings, eol: Vec<u8>) -> Result<Self, Error> {
        let base = GenericDriver::open(settings)?;

        Ok(Self {
            base: base,
            eol: eol,
        })
    }
}

#[async_trait]
impl SerialDriver for Driver {
    async fn write(&mut self, command: &[u8]) -> Result<usize, Error> {
        //
        // Append EOL to the command
        let mut internal_cmd = command.to_vec();
        internal_cmd.extend_from_slice(&self.eol);

        // Write
        let count = self.base.write_time_locked(internal_cmd.as_slice()).await?;

        return Ok(count - self.eol.len());
    }

    /// Lock the connector to write a command then wait for the answers
    ///
    async fn write_then_read(
        &mut self,
        command: &[u8],
        response: &mut [u8],
    ) -> Result<usize, Error> {
        //
        // Append EOL to the command
        let mut internal_cmd = command.to_vec();
        internal_cmd.extend_from_slice(&self.eol);

        // Write
        self.base.write_time_locked(internal_cmd.as_slice()).await?;

        // Read
        let count = self.base.read_until_timeout(response, &self.eol).await?;

        return Ok(count - self.eol.len());
    }
}
