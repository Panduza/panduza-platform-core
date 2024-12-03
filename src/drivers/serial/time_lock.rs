use std::time::Duration;

use super::{common, Settings as SerialSettings};
use crate::protocol::AsciiCmdRespProtocol;
use crate::{format_driver_error, log_trace, DriverLogger, Error};
use async_trait::async_trait;
use serial2_tokio::SerialPort;
use tokio::io::AsyncReadExt;
use tokio::time::sleep;

pub struct TimeLock {
    pub duration: tokio::time::Duration,
    pub t0: tokio::time::Instant,
}

/// # Timelock Serial Driver
///
/// This driver must be used only for very broken devices that does not send EOF or \n
/// at the end of there message packets
///
pub struct Driver {
    ///
    /// To help data logging inside the driver
    ///
    logger: DriverLogger,
    ///
    /// The serial port object
    ///
    port: SerialPort,
    ///
    /// A buffer to read incoming data
    ///
    read_buffer: [u8; 1024],
    ///
    ///
    ///
    time_lock_duration: Duration,
    ///
    ///
    ///
    time_lock: Option<TimeLock>,
}

impl Driver {
    /// Create a new instance of the driver
    ///
    pub fn open(settings: &SerialSettings) -> Result<Self, Error> {
        //
        // Open the port
        let (logger, port) = common::open(settings)?;
        Ok(Self {
            logger: logger,
            port: port,
            read_buffer: [0; 1024],
            time_lock_duration: settings
                .time_lock_duration
                .unwrap_or(Duration::from_secs(1)),
            time_lock: None,
        })
    }

    /// Write a command on the serial stream
    ///
    pub async fn write_time_locked(&mut self, command: &[u8]) -> Result<usize, Error> {
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
            .port
            .write(command)
            .await
            .map_err(|e| format_driver_error!("Unable to write on serial port: {}", e));

        // Set the time lock
        self.time_lock = Some(TimeLock {
            duration: self.time_lock_duration,
            t0: tokio::time::Instant::now(),
        });

        return write_result;
    }

    ///
    ///
    ///
    async fn read_one_by_one(&mut self) -> Result<usize, Error> {
        let n = 0;
        loop {
            let mut single_buf = [0u8; 1];

            // timeout here with small time
            let operation_result = tokio::time::timeout(
                Duration::from_millis(5),
                self.port.read_exact(&mut single_buf),
            )
            .await;

            match operation_result {
                Ok(read_result) => {
                    if let Err(e) = read_result {
                        return format_driver_error!(
                            "Unable to read one more on serial port {:?}",
                            e
                        );
                    }
                    self.read_buffer[n] = single_buf[0];
                    n += 1;
                }
                Err(_) => return Ok(n),
            }

            //
            // Debug
            log_trace!(self.logger, "Read one {:?}", self.read_buffer[..n].to_vec());
        }
    }

    /// Lock the connector to write a command then wait for the answers
    ///
    pub async fn write_then_read_after(&mut self, command: &[u8]) -> Result<usize, Error> {
        // Write
        self.write_time_locked(command).await?;

        //
        sleep(self.time_lock_duration).await;

        self.read_one_by_one().await
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
        self.write_time_locked(command_buffer.as_slice()).await?;
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
            .write_then_read_after(command_buffer.as_slice())
            .await?;

        //
        // Build response string
        let string_slice = String::from_utf8(self.read_buffer[..count].to_vec()).unwrap();
        return Ok(string_slice.to_string());
    }
}
