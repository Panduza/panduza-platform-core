use std::time::Duration;

use super::generic::Driver as GenericDriver;
use super::Settings as SerialSettings;
use crate::Error;
use tokio::time::timeout;

use crate::format_driver_error;
use crate::log_debug;
use crate::log_info;

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

    pub async fn write(&mut self, command: &[u8]) -> Result<usize, Error> {
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
    pub async fn write_then_read(
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

    // pub async fn get_idn(&mut self) -> Result<String, Error> {
    //     let mut response: [u8; 512] = [0; 512];

    //     // let cmd = "*IDN?\n".as_bytes();
    //     let cmd = "*IDN?".as_bytes();

    //     let count = self
    //         .driver
    //         .write_then_read_until(cmd, &mut response, '\n' as u8)
    //         .await?;

    //     println!("{:?}", response[..count].to_vec());

    //     // count -1 because we remove the '\n'
    //     let string_slice = String::from_utf8(response[..count - 1].to_vec()).unwrap();
    //     let string = string_slice.to_string();

    //     Ok(string)
    // }
}
