use crate::{log_debug, Error, Instance};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;

#[async_trait]
///
/// Trait for driver that can read *IDN?
///
pub trait IDNProvider: Sync + Send {
    ///
    /// Send a command and return the response
    ///
    async fn read_idn(&mut self) -> Result<String, Error>;
}

///
/// Mount the identity attribute
///
pub async fn mount(
    mut instance: Instance,
    driver: Arc<Mutex<dyn IDNProvider>>,
) -> Result<(), Error> {
    //
    // Create the local logger
    let logger = instance.logger.new_attribute_logger("", "identity");
    log_debug!(logger, "Mounting...");

    //
    // Create attribute
    let att_identity = instance
        .create_attribute("identity")
        .with_ro()
        .with_info("Identity string of the device")
        .finish_as_string()
        .await?;

    //
    // Just init
    let idn = driver.lock().await.read_idn().await?;
    log_debug!(logger, "IDN ({:?})", &idn);
    att_identity.set(idn).await?;

    //
    // End
    log_debug!(logger, "Mounting => OK");
    Ok(())
}
