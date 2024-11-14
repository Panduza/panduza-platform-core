pub mod eol;
pub mod generic;
pub mod settings;
pub mod time_lock;
use async_trait::async_trait;

pub use settings::Settings;

use crate::Error;

#[async_trait]
pub trait SerialDriver {
    async fn write(&mut self, command: &[u8]) -> Result<usize, Error>;

    async fn write_then_read(
        &mut self,
        command: &[u8],
        response: &mut [u8],
    ) -> Result<usize, Error>;
}
