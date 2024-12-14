use crate::{log_debug, spawn_on_command, Container, Error, Instance, Logger, StringAttServer};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;

#[async_trait]
///
/// Protocol in which we send a text command and the device respond with another text
///
pub trait ReplProtocol: Sync + Send {
    ///
    /// Evaluate the command and return the response
    ///
    async fn eval(&mut self, command: String) -> Result<String, Error>;
}

///
/// Mount the identity attribute
///
pub async fn mount<A: Into<String>>(
    class_name: A,
    mut instance: Instance,
    repl_driver: Arc<Mutex<dyn ReplProtocol>>,
) -> Result<(), Error> {
    //
    //
    let class_name_string = class_name.into();

    //
    // Create the local logger
    let logger = instance.logger.new_for_attribute(None, &class_name_string);
    log_debug!(logger, "Mounting...");

    //
    //
    let mut class_repl = instance
        .create_class(&class_name_string)
        .with_tag("REPL")
        .finish()
        .await;

    let att_command = class_repl
        .create_attribute("command")
        .with_wo()
        .finish_as_string()
        .await?;

    let att_response = class_repl
        .create_attribute("response")
        .with_ro()
        .finish_as_string()
        .await?;

    //
    // Execute action on each command received
    let logger_2 = instance.logger.new_for_attribute(None, "command");
    let att_command_2 = att_command.clone();
    let att_response_2 = att_response.clone();
    spawn_on_command!(
        "on_command => relp",
        instance,
        att_command_2,
        on_command(
            logger_2.clone(),
            att_command_2.clone(),
            att_response_2.clone(),
            repl_driver.clone()
        )
    );

    //
    // End
    log_debug!(logger, "Mounting => OK");
    Ok(())
}

///
/// On command callback
///
async fn on_command(
    logger: Logger,
    mut att_command: StringAttServer,
    att_response: StringAttServer,
    repl_driver: Arc<Mutex<dyn ReplProtocol>>,
) -> Result<(), Error> {
    while let Some(command) = att_command.pop_cmd().await {
        //
        // Log
        log_debug!(logger, "Command received '{:?}'", command);
        let response = repl_driver.lock().await.eval(command).await?;
        att_response.set(response).await?;
    }
    Ok(())
}
