use super::Settings as SerialSettings;
use crate::DriverLogger;
use crate::Error;
use serial2_tokio::SerialPort;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;
use tokio::time::sleep;
use tokio::time::timeout;
use tokio_serial::SerialStream;

/// # Serial SLIP Driver
///
/// The goal of this driver is to manage the stack SLIP over SERIAL
///
/// ## What is SLIP ?
///
/// - [Wikipedia](https://en.wikipedia.org/wiki/Serial_Line_Internet_Protocol)
///
/// The Serial Line Internet Protocol (SLIP) is an encapsulation of the
/// Internet Protocol designed to work over serial ports and router connections.
/// It is documented in RFC 1055.
///
/// ## Why SLIP ?
///
/// This protocol helps splitting serial stream into packets.
/// You could just use EOL character driver but if you may have the
/// EOL char inside your payload data it becomes a problem.
/// SLIP works like EOL but provides a mecanism to avoid this problem
/// by encoding the payload with a simple et fast method.
///
pub struct SerialSlipDriver {
    ///
    ///
    ///
    pub logger: DriverLogger,

    // Serial settings
    settings: SerialSettings,

    pub port: SerialPort,

    // Accumulated incoming data buffer
    in_buf: [u8; 512],
    // Keep track of number of data in the buffer
    in_buf_size: usize,
}

/// Connector is just a mutex protected driver
///
pub type Connector = Arc<Mutex<Driver>>;

impl Driver {
    /// Create a new instance of the driver
    ///
    pub fn new(settings: &SerialSettings) -> Self {
        // Get the port name safely
        let port_name = settings
            .port_name
            .as_ref()
            .map(|val| val.clone())
            .unwrap_or("undefined".to_string())
            .clone();

        // Create instance
        Driver {
            logger: DriverLogger::new("serial", "slip", port_name),
            settings: settings.clone(),
            serial_stream: None,
            time_lock: None,
            in_buf: [0u8; 512],
            in_buf_size: 0,
        }
    }

    /// Convert the driver into a connector
    ///
    pub fn into_connector(self) -> Connector {
        Arc::new(Mutex::new(self))
    }

    /// Initialize the driver
    ///
    pub async fn init(&mut self) -> Result<(), Error> {
        //
        //
        self.logger.info("init");

        // Internal driver already initialized by an other entity => OK
        if self.serial_stream.is_some() {
            self.logger.warn("already init");
            return Ok(());
        }

        // Get the port name
        let port_name =
            self.settings.port_name.as_ref().ok_or_else(|| {
                Error::BadSettings("Port name is not set in settings".to_string())
            })?;

        // Setup builder
        let serial_builder = tokio_serial::new(port_name, self.settings.baudrate)
            .data_bits(self.settings.data_bits)
            .stop_bits(self.settings.stop_bits)
            .parity(self.settings.parity)
            .flow_control(self.settings.flow_control);

        // Build the stream
        self.serial_stream = Some(
            SerialStream::open(&serial_builder)
                .map_err(|e| Error::BadSettings(format!("Unable to open serial stream: {}", e)))?,
        );

        self.logger.info("stream ready");

        Ok(())
    }

    /// Write a command on the serial stream
    ///
    async fn write_time_locked(&mut self, command: &[u8]) -> Result<usize, Error> {
        // Check if a time lock is set
        if let Some(lock) = self.time_lock.as_mut() {
            let elapsed = tokio::time::Instant::now() - lock.t0;
            if elapsed < lock.duration {
                let wait_time = lock.duration - elapsed;
                tokio::time::sleep(wait_time).await;
            }
            self.time_lock = None;
        }

        // Send the command
        let write_result = self
            .serial_stream
            .as_mut()
            .ok_or_else(|| Error::BadSettings("No serial stream".to_string()))?
            .write(command)
            .await
            .map_err(|e| Error::BadSettings(format!("Unable to write on serial stream: {}", e)));

        // Set the time lock
        if let Some(duration) = self.settings.time_lock_duration {
            self.time_lock = Some(TimeLock {
                duration: duration,
                t0: tokio::time::Instant::now(),
            });
        }

        return write_result;
    }

    /// Lock the connector to write a command then wait for the answers
    ///
    pub async fn write_then_read(
        &mut self,
        command: &[u8],
        response: &mut [u8],
    ) -> Result<usize, Error> {
        match self.settings.read_timeout {
            // If the timeout is set, use it
            Some(timeout_value) => {
                return Ok(
                    timeout(timeout_value, self.__write_then_read(command, response))
                        .await
                        .map_err(|e| Error::BadSettings(format!("Timeout reading {:?}", e)))??,
                );
            }
            // Else good luck !
            None => {
                return Ok(self.__write_then_read(command, response).await?);
            }
        }
    }

    /// This operation is not provided to the public interface
    /// User must use the timeout version for safety on the platform
    ///
    async fn __write_then_read(
        &mut self,
        command: &[u8],
        response: &mut [u8],
    ) -> Result<usize, Error> {
        // Prepare SLIP encoding
        // Prepare a buffer of 1024 Bytes (need to be change later TODO)
        // and prepare the encoder object
        let mut encoded_command = [0u8; 1024];
        let mut slip_encoder = serial_line_ip::Encoder::new();

        // Encode the command
        let mut totals = slip_encoder
            .encode(command, &mut encoded_command)
            .map_err(|e| Error::BadSettings(format!("Unable to encode command: {:?}", e)))?;

        // Finalise the encoding
        totals += slip_encoder
            .finish(&mut encoded_command[totals.written..])
            .map_err(|e| {
                Error::BadSettings(format!("Unable to finsh command encoding: {:?}", e))
            })?;

        // Write command slip encoded
        self.write_time_locked(&encoded_command[..totals.written])
            .await?;

        // Read the response until "end"
        loop {
            // Read a chunck
            self.in_buf_size += self
                .serial_stream
                .as_mut()
                .ok_or_else(|| Error::BadSettings("No serial stream".to_string()))?
                .read(&mut self.in_buf[self.in_buf_size..])
                .await
                .map_err(|e| {
                    Error::BadSettings(format!("Unable to read on serial stream {:?}", e))
                })?;

            // Try decoding
            let mut slip_decoder = serial_line_ip::Decoder::new();
            let (total_decoded, out_slice, end) = slip_decoder
                .decode(&self.in_buf[..self.in_buf_size], response)
                .map_err(|e| Error::BadSettings(format!("Unable to decode response: {:?}", e)))?;

            // Reste counter
            self.in_buf_size -= total_decoded;

            // If a valid packet has been found, then we must return the out_slice len
            //      which is the len a the decoded data
            // Not '_total_decoded'
            //      because it is the number of byte processed from the encoded buffer
            if end {
                return Ok(out_slice.len());
            }
        }
    }
}

impl Drop for Driver {
    fn drop(&mut self) {
        // Close the serial stream
        self.logger.warn("Closing serial stream");
        self.serial_stream = None;
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slip_decode() {
        const SLIP_ENCODED: [u8; 8] = [0xc0, 0x01, 0x02, 0x03, 0x04, 0x05, 0xc0, 0x04];
        const DATA: [u8; 5] = [0x01, 0x02, 0x03, 0x04, 0x05];

        let mut output: [u8; 32] = [0; 32];
        let mut slip = serial_line_ip::Decoder::new();

        let (input_bytes_processed, output_slice, is_end_of_packet) =
            slip.decode(&SLIP_ENCODED, &mut output).unwrap();

        assert_eq!(7, input_bytes_processed);
        assert_eq!(&DATA, output_slice);
        assert_eq!(true, is_end_of_packet);
    }
}
