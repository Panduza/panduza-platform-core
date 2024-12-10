use crate::{log_debug, Error, Instance};
use std::sync::Arc;
use tokio::sync::Mutex;

// use crate::Error;
use async_trait::async_trait;

#[async_trait]
///
///
///
pub trait IDNProvider: Sync + Send {
    ///
    /// Send a command and return the response
    ///
    async fn read_idn(&mut self) -> Result<String, Error>;
}

///
///
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
        .with_info("Identity string of the power supply")
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
