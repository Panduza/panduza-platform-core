// usage
use super::time_lock::TimeLock;
use super::Settings as SerialSettings;
use crate::DriverLogger;
use crate::Error;
use serial2_tokio::SerialPort;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::AsyncReadExt;
use tokio::sync::Mutex;
use tokio::time::sleep;

use crate::format_driver_error;
use crate::log_info;

/// Serial GENERIC driver
///
pub struct Driver {
    ///
    ///
    ///
    logger: DriverLogger,

    ///
    ///
    ///
    settings: SerialSettings,

    port: SerialPort,

    time_lock: Option<TimeLock>,
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
        let logger = DriverLogger::new("serial", "generic", &port_name);
        log_info!(logger, "Open serial driver {:?}", &port_name);

        let port = SerialPort::open(&port_name, settings.baudrate)
            .map_err(|e| format_driver_error!("Port {:?} {:?}", &port_name, e))?;

        // Create instance
        Ok(Driver {
            logger: logger,
            settings: settings.clone(),
            port: port,
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
        // Write
        self.write_time_locked(command).await?;

        // Read the response
        self.port
            .read(response)
            .await
            .map_err(|e| format_driver_error!("Unable to read on serial port {:?}", e))
    }

    /// Lock the connector to write a command then wait for the answers
    ///
    pub async fn write_then_read_during(
        &mut self,
        command: &[u8],
        response: &mut [u8],
        duration: Duration,
    ) -> Result<usize, Error> {
        // Write
        self.write_time_locked(command).await?;

        sleep(duration).await;

        // Read the response
        self.port
            .read(response)
            .await
            .map_err(|e| format_driver_error!("Unable to read on serial port {:?}", e))
    }

    ///
    ///
    pub async fn write_then_read_until(
        &mut self,
        command: &[u8],
        response: &mut [u8],
        end: u8,
    ) -> Result<usize, Error> {
        // Write
        self.write_time_locked(command).await?;

        // Read the response until "end"
        let mut n = 0;
        loop {
            let mut single_buf = [0u8; 1];
            self.port
                .read_exact(&mut single_buf)
                .await
                .map_err(|e| format_driver_error!("Unable to read on serial port {:?}", e))?;
            response[n] = single_buf[0];
            n += 1;
            if single_buf[0] == end {
                break;
            }
        }
        Ok(n)
    }
}
