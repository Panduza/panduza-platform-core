use crate::Error;
use async_trait::async_trait;

#[async_trait]
///
/// Protocol in which we send a text command and the device respond with another text
///
pub trait CommandResponseProtocol: Sync + Send {
    ///
    /// Just send a command and does not expect any response
    ///
    async fn send(&mut self, command: &String) -> Result<(), Error>;

    ///
    /// Send a command and return the response
    ///
    async fn ask(&mut self, command: &String) -> Result<String, Error>;
}
