use super::Settings as SerialSettings;
use crate::format_driver_error;
use crate::log_info;
use crate::DriverLogger;
use crate::Error;
use serial2_tokio::SerialPort;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::timeout;

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
pub struct Driver {
    ///
    ///
    ///
    pub logger: DriverLogger,

    ///
    /// Serial settings
    ///
    settings: SerialSettings,

    ///
    ///
    ///
    pub port: SerialPort,

    ///
    /// Accumulated incoming data buffer
    ///
    in_buf: [u8; 2048],

    ///
    /// Keep track of number of data in the buffer
    ///
    in_buf_size: usize,
}

/// Connector is just a mutex protected driver
///
pub type Connector = Arc<Mutex<Driver>>;

impl Driver {
    /// Create a new instance of the driver
    ///
    pub fn open(settings: &SerialSettings) -> Result<Self, Error> {
        // Get the port name safely
        let port_name = settings
            .port_name
            .as_ref()
            .map(|val| val.clone())
            .unwrap_or("undefined".to_string())
            .clone();

        //
        // Prepare logger
        let logger = DriverLogger::new("serial", "slip", &port_name);
        log_info!(logger, "Opening serial driver {:?}...", &port_name);

        //
        // Open port
        let port = SerialPort::open(&port_name, settings.baudrate)
            .map_err(|e| format_driver_error!("Port {:?} {:?}", &port_name, e))?;

        //
        // Info logs
        log_info!(logger, "Open success !");

        // Create instance
        Ok(Driver {
            logger: logger,
            settings: settings.clone(),
            port: port,
            in_buf: [0u8; 2048],
            in_buf_size: 0,
        })
    }

    /// Lock the connector to write a command then wait for the answers
    ///
    pub async fn write_then_read(
        &mut self,
        command: &[u8],
        response: &mut [u8],
    ) -> Result<usize, Error> {
        Ok(timeout(
            self.settings.read_timeout,
            self.__write_then_read(command, response),
        )
        .await
        .map_err(|e| format_driver_error!("Timeout reading {:?}", e))??)
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
            .map_err(|e| format_driver_error!("Unable to encode command: {:?}", e))?;

        // Finalise the encoding
        totals += slip_encoder
            .finish(&mut encoded_command[totals.written..])
            .map_err(|e| format_driver_error!("Unable to finsh command encoding: {:?}", e))?;

        // Send the command
        let _write_result = self
            .port
            .write(command)
            .await
            .map_err(|e| format_driver_error!("Unable to write on serial stream: {}", e))?;

        // Read the response until "end"
        loop {
            // Read a chunck
            self.in_buf_size += self
                .port
                .read(&mut self.in_buf[self.in_buf_size..])
                .await
                .map_err(|e| format_driver_error!("Unable to read on serial stream {:?}", e))?;

            // Try decoding
            let mut slip_decoder = serial_line_ip::Decoder::new();
            let (total_decoded, out_slice, end) = slip_decoder
                .decode(&self.in_buf[..self.in_buf_size], response)
                .map_err(|e| format_driver_error!("Unable to decode response: {:?}", e))?;

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
