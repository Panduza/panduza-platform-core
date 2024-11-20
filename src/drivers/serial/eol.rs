use super::generic::Driver as GenericDriver;
use super::Settings as SerialSettings;
use crate::Error;

use crate::protocol::CommandResponseProtocol;
use async_trait::async_trait;

///
///
pub struct Driver {
    ///
    ///
    ///
    base: GenericDriver,

    ///
    ///
    ///
    eol: Vec<u8>,

    ///
    ///
    ///
    read_buffer: [u8; 1024],
}

impl Driver {
    /// Create a new instance of the driver
    ///
    pub fn open(settings: &SerialSettings, eol: Vec<u8>) -> Result<Self, Error> {
        let base = GenericDriver::open(settings)?;

        Ok(Self {
            base: base,
            eol: eol,
            read_buffer: [0; 1024],
        })
    }
}

#[async_trait]
impl CommandResponseProtocol for Driver {
    ///
    ///
    ///
    async fn send(&mut self, command: &String) -> Result<(), Error> {
        //
        // Append EOL to the command
        let mut command_buffer = command.clone().into_bytes();
        command_buffer.extend(&self.eol);

        //
        // Write
        self.base
            .write_time_locked(command_buffer.as_slice())
            .await?;
        Ok(())
    }

    ///
    ///
    ///
    async fn ask(&mut self, command: &String) -> Result<String, Error> {
        //
        // Append EOL to the command
        let mut command_buffer = command.clone().into_bytes();
        command_buffer.extend(&self.eol);

        //
        // Write
        self.base
            .write_time_locked(command_buffer.as_slice())
            .await?;

        //
        // Read
        let count = self
            .base
            .read_until_timeout(&mut self.read_buffer, &self.eol)
            .await?;

        //
        // Build response string
        let string_slice =
            String::from_utf8(self.read_buffer[..count - self.eol.len()].to_vec()).unwrap();
        return Ok(string_slice.to_string());
    }
}
