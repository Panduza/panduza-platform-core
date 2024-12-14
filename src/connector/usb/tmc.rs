use super::Settings as UsbSettings;
use crate::protocol::AsciiCmdRespProtocol;
use crate::std::class::repl::ReplProtocol;
use crate::{format_driver_error, log_debug, log_trace, DriverLogger, Error};
use async_trait::async_trait;
use futures::executor::block_on;
use nusb::Interface as UsbInterface;
use nusb::{transfer::Direction, transfer::EndpointType, Interface};
use serial2_tokio::SerialPort;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::timeout;
use usbtmc_message::Sequencer;

///
///
pub struct Driver {
    ///
    /// To help data logging inside the driver
    ///
    logger: DriverLogger,

    usb_interface: UsbInterface,

    endpoint_in: u8,
    endpoint_out: u8,
    max_packet_size: usize,
}

impl Driver {
    ///
    ///
    pub fn into_arc_mutex(self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(self))
    }

    /// Create a new instance of the driver
    ///
    pub fn open(
        settings: &UsbSettings,
        endpoint_in: u8,
        endpoint_out: u8,
        max_packet_size: usize,
    ) -> Result<Self, Error> {
        //
        // Prepare logger
        let logger = DriverLogger::new("usb", "tmc", "");
        // log_debug!(logger, "Opening serial driver {:?}...", &port_name);

        let dev = settings.find_usb_device();

        let device = match dev.unwrap().open() {
            Ok(val) => val,
            Err(_e) => return Err(format_driver_error!("Unable to open USB device")),
        };

        let interface = match device.claim_interface(0) {
            Ok(val) => Some(val),
            Err(_e) => {
                return Err(format_driver_error!(
                    "Unable to create USB device interface"
                ))
            }
        };

        Ok(Self {
            logger: logger,
            usb_interface: interface.unwrap(),
            endpoint_in: endpoint_in,
            endpoint_out: endpoint_out,
            max_packet_size: max_packet_size,
        })
    }
}

#[async_trait]
impl ReplProtocol for Driver {
    ///
    /// Send a command and return the response
    ///
    async fn eval(&mut self, command: String) -> Result<String, Error> {
        // Create a sequencer with a max_sequence_length of 64 (depend on your device)
        let mut sequencer = Sequencer::new(self.max_packet_size as u32);

        // Create a message sequence from a command
        let sequence = sequencer.command_to_message_sequence(command.clone());

        // Send the sequence on the usb
        for i in 0..sequence.len() {
            let message = sequence[i].to_vec();
            // SEND TO USB
            match block_on(
                self.usb_interface
                    .bulk_out(self.endpoint_out, message.to_vec()),
            )
            .into_result()
            {
                Ok(val) => val,
                Err(_e) => return Err(format_driver_error!("Unable to write on USB")),
            };
        }

        let response = nusb::transfer::RequestBuffer::new(self.max_packet_size);

        // Receive data form the usb
        let data =
            match block_on(self.usb_interface.bulk_in(self.endpoint_in, response)).into_result() {
                Ok(val) => val,
                Err(_e) => return Err(format_driver_error!("Unable to read on USB")),
            };

        // Parse the received data
        let msg = usbtmc_message::BulkInMessage::from_u8_array(&data);

        Ok(msg.payload_as_string())
    }
}

#[async_trait]
impl AsciiCmdRespProtocol for Driver {
    ///
    ///
    ///
    async fn send(&mut self, command: &String) -> Result<(), Error> {
        // //
        // // Append EOL to the command
        // let mut command_buffer = command.clone().into_bytes();
        // command_buffer.extend(&self.eol);

        // //
        // // Write
        // self.port
        //     .write(command_buffer.as_slice())
        //     .await
        //     .map_err(|e| format_driver_error!("Unable to write on serial port: {:?}", e))?;

        Ok(())
    }

    ///
    ///
    ///
    async fn ask(&mut self, command: &String) -> Result<String, Error> {
        // Create a sequencer with a max_sequence_length of 64 (depend on your device)
        let mut sequencer = Sequencer::new(self.max_packet_size as u32);

        // Create a message sequence from a command
        let sequence = sequencer.command_to_message_sequence(command.clone());

        // Send the sequence on the usb
        for i in 0..sequence.len() {
            let message = sequence[i].to_vec();
            // SEND TO USB
            match block_on(
                self.usb_interface
                    .bulk_out(self.endpoint_out, message.to_vec()),
            )
            .into_result()
            {
                Ok(val) => val,
                Err(_e) => return Err(format_driver_error!("Unable to write on USB")),
            };
        }

        let response = nusb::transfer::RequestBuffer::new(self.max_packet_size);

        // Receive data form the usb
        let data =
            match block_on(self.usb_interface.bulk_in(self.endpoint_in, response)).into_result() {
                Ok(val) => val,
                Err(_e) => return Err(format_driver_error!("Unable to read on USB")),
            };

        // Parse the received data
        let msg = usbtmc_message::BulkInMessage::from_u8_array(&data);

        Ok(msg.payload_as_string())
    }
}
