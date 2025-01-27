use super::Settings as UsbSettings;
use crate::protocol::AsciiCmdRespProtocol;
use crate::std::class::repl::ReplProtocol;
use crate::{format_driver_error, log_trace, log_warn, Error, Logger};
use async_trait::async_trait;
use byteorder::{ByteOrder, LittleEndian};

use futures::executor::block_on;
use nusb::Interface as UsbInterface;
// use nusb::{transfer::Direction, transfer::EndpointType, Interface};
// use serial2_tokio::SerialPort;
use std::sync::Arc;
// use std::time::Duration;
use tokio::sync::Mutex;
// use tokio::time::timeout;
use usbtmc_message::Sequencer;

///
///
#[derive(Copy, Clone)]
pub enum MsgID {
    DevDepMsgOut = 1,
    DevDepMsgIn = 2,
    VendorSpecificOut = 126,
    VendorSpecificIn = 127,
}

impl MsgID {
    pub const DEV_DEP_MSG_OUT: MsgID = MsgID::DevDepMsgOut;

    pub const REQUEST_DEV_DEP_MSG_IN: MsgID = MsgID::DevDepMsgIn;
    pub const DEV_DEP_MSG_IN: MsgID = MsgID::DevDepMsgIn;

    pub const VENDOR_SPECIFIC_OUT: MsgID = MsgID::VendorSpecificOut;

    pub const REQUEST_VENDOR_SPECIFIC_IN: MsgID = MsgID::VendorSpecificIn;
    pub const VENDOR_SPECIFIC_IN: MsgID = MsgID::VendorSpecificIn;

    pub fn from_u8_array(value: u8) -> Option<MsgID> {
        match value {
            1 => Some(MsgID::DevDepMsgOut),
            2 => Some(MsgID::DevDepMsgIn),
            126 => Some(MsgID::VendorSpecificOut),
            127 => Some(MsgID::VendorSpecificIn),
            _ => None,
        }
    }
}

///
///
pub struct Driver {
    ///
    /// To help data logging inside the driver
    ///
    logger: Logger,

    usb_interface: UsbInterface,

    endpoint_in: u8,
    endpoint_out: u8,
    max_packet_size_in: usize,
    max_packet_size_out: usize,
}

impl Driver {
    ///
    ///
    pub fn into_arc_mutex(self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(self))
    }

    /// Create a new instance of the driver
    ///
    pub fn open(settings: &UsbSettings) -> Result<Self, Error> {
        //
        // Prepare logger
        let logger = Logger::new_for_driver("usb", "tmc");

        // Find the USB device
        let dev = settings.find_usb_device();

        let device: nusb::Device = match dev.unwrap().open() {
            Ok(val) => val,
            Err(_e) => return Err(format_driver_error!("Unable to open USB device")),
        };

        let interface: Option<UsbInterface> = match device.claim_interface(0) {
            Ok(val) => Some(val),
            Err(_e) => {
                return Err(format_driver_error!(
                    "Unable to create USB device interface"
                ))
            }
        };

        // Find the IN endpoint in the configuration
        let (endpoint_in, max_packet_size_in) =
            Self::find_endpoint_in_config(&logger, interface.as_ref().unwrap()).unwrap();

        // Find the OUT endpoint in the configuration
        let (endpoint_out, max_packet_size_out) =
            Self::find_endpoint_out_config(&logger, interface.as_ref().unwrap()).unwrap();

        // let max_packet_size = endpoint_descriptor.max_packet_size() as usize;

        Ok(Self {
            logger: logger,
            usb_interface: interface.unwrap(),
            endpoint_in: endpoint_in,
            endpoint_out: endpoint_out,
            max_packet_size_in: max_packet_size_in,
            max_packet_size_out: max_packet_size_out,
        })
    }

    /// Find the in endpoint IN the configuration
    ///
    fn find_endpoint_in_config(
        logger: &Logger,
        interface: &nusb::Interface,
    ) -> Result<(u8, usize), Error> {
        for desc in interface.descriptors() {
            for endpoint in desc.endpoints() {
                if endpoint.direction() == nusb::transfer::Direction::In
                    && endpoint.transfer_type() == nusb::transfer::EndpointType::Bulk
                {
                    // If the endpoint is not 0x81, log a warning
                    // and continue, it can be a problem
                    if endpoint.address() != 0x81 {
                        log_warn!(
                            logger,
                            "Endpoint address is not 0x81, but {}",
                            endpoint.address()
                        );
                    }

                    // Trace the endpoint found and return configuration
                    log_trace!(logger, "In Endpoint found: {:?}", endpoint);
                    return Ok((endpoint.address(), endpoint.max_packet_size() as usize));
                }
            }
        }

        // If no endpoint is found, return an error
        Err(format_driver_error!(
            "Unable to find the IN endpoint in the USB device configuration"
        ))
    }

    /// Find the in endpoint OUT the configuration
    ///
    fn find_endpoint_out_config(
        logger: &Logger,
        interface: &nusb::Interface,
    ) -> Result<(u8, usize), Error> {
        for desc in interface.descriptors() {
            for endpoint in desc.endpoints() {
                if endpoint.direction() == nusb::transfer::Direction::Out
                    && endpoint.transfer_type() == nusb::transfer::EndpointType::Bulk
                {
                    // If the endpoint is not 0x02, log a warning
                    // and continue, it can be a problem
                    if endpoint.address() != 0x02 {
                        log_warn!(
                            logger,
                            "Endpoint address is not 0x02, but {}",
                            endpoint.address()
                        );
                    }

                    // Trace the endpoint found and return configuration
                    log_trace!(logger, "Out Endpoint found: {:?}", endpoint);
                    return Ok((endpoint.address(), endpoint.max_packet_size() as usize));
                }
            }
        }

        // If no endpoint is found, return an error
        Err(format_driver_error!(
            "Unable to find the OUT endpoint in the USB device configuration"
        ))
    }

    ///
    ///
    fn parse_bulk_in_header(&self, data: &Vec<u8>) -> Result<usize, Error> {
        // log
        log_trace!(self.logger, "msg id: {}", data[0]);

        let transfer_size = LittleEndian::read_u32(&data[4..8]) as usize;

        Ok(transfer_size)
    }
}

#[async_trait]
impl ReplProtocol for Driver {
    ///
    /// Send a command and return the response
    ///
    async fn eval(&mut self, command: String) -> Result<String, Error> {
        // log
        log_trace!(self.logger, "Eval: {}", command);

        // let factor = 4;

        // Create a sequencer with a max_sequence_length of 64 (depend on your device)
        let mut sequencer = Sequencer::new(self.max_packet_size_out as u32);

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

        let mut is_first: bool = true;
        let mut remaining_data = 0;
        let mut complete_data = Vec::new();
        let mut is_eom = false;

        while !is_eom {
            let response = nusb::transfer::RequestBuffer::new(self.max_packet_size_in);

            // log
            log_trace!(self.logger, "Wait for bulk_in data...");

            // Receive data from the usb
            match tokio::time::timeout(
                std::time::Duration::from_secs(1),
                self.usb_interface.bulk_in(self.endpoint_in, response),
            )
            .await
            {
                // TODO
                // Read the header first time then read until all data is received
                Ok(val) => match val.into_result() {
                    Ok(data) => {
                        //
                        //
                        if is_first {
                            // Parse the received data
                            let transfer_size = self.parse_bulk_in_header(&data).unwrap();

                            log_trace!(self.logger, "FIRST !!!! Data {:?}", transfer_size);

                            is_first = false;
                            remaining_data = transfer_size + 12; // 12 => bulkin usbtmc header size
                        }

                        // log
                        log_trace!(self.logger, "Data received (pack len:{:?}): ", data.len());

                        // log
                        log_trace!(
                            self.logger,
                            "Data received (len:{:?}): {:?}",
                            remaining_data,
                            data
                        );

                        if remaining_data >= data.len() {
                            remaining_data -= data.len();

                            // Append the payload to the complete data
                            complete_data.extend(data);
                        } else {
                            complete_data.extend(&data[..remaining_data]);
                            remaining_data = 0;
                        }

                        // Check if this is the end of the message
                        if remaining_data > 0 {
                            is_eom = false;
                        } else {
                            is_eom = true;
                        }
                    }
                    Err(_e) => return Err(format_driver_error!("Unable to read on USB")),
                },
                Err(_) => {
                    log_trace!(self.logger, "Timeout while reading from USB");
                    return Ok("Timeout while reading from USB".to_string());
                }
            };
        }

        // Parse the received data
        let msg = usbtmc_message::BulkInMessage::from_u8_array(&complete_data);

        log_trace!(
            self.logger,
            "end Data {:?}: {:?}",
            msg.bulk_in_header().is_eom(),
            msg.bulk_in_header().transfer_size()
        );

        // Return the payload as a string no matter what
        Ok(msg.payload_as_string())
    }
}

#[async_trait]
impl AsciiCmdRespProtocol for Driver {
    ///
    ///
    ///
    async fn send(&mut self, _command: &String) -> Result<(), Error> {
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
        let mut sequencer = Sequencer::new(self.max_packet_size_out as u32);

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

        let response = nusb::transfer::RequestBuffer::new(self.max_packet_size_in);

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
