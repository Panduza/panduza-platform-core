pub struct TimeLock {
    pub duration: tokio::time::Duration,
    pub t0: tokio::time::Instant,
}

use super::generic::Driver as GenericDriver;
use super::Settings as SerialSettings;
use crate::Error;

use crate::protocol::AsciiCmdRespProtocol;
use async_trait::async_trait;

///
///
pub struct Driver {
    ///
    ///
    ///
    base: GenericDriver,

    read_buffer: [u8; 1024],
}

impl Driver {
    /// Create a new instance of the driver
    ///
    pub fn open(settings: &SerialSettings) -> Result<Self, Error> {
        let base = GenericDriver::open(settings)?;

        Ok(Self {
            base: base,
            read_buffer: [0; 1024],
        })
    }
}

#[async_trait]
impl AsciiCmdRespProtocol for Driver {
    ///
    ///
    ///
    async fn send(&mut self, command: &String) -> Result<(), Error> {
        //
        // Append EOL to the command
        let command_buffer = command.clone().into_bytes();

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
        let command_buffer = command.clone().into_bytes();

        //
        // Read
        let count = self
            .base
            .write_then_read_after(
                command_buffer.as_slice(),
                &mut self.read_buffer,
                self.base.settings.read_timeout,
            )
            .await?;

        //
        // Build response string
        let string_slice = String::from_utf8(self.read_buffer[..count].to_vec()).unwrap();
        return Ok(string_slice.to_string());
    }
}
