// use super::AttOnlyMsgAttInner;
// use crate::{AttributeBuilder, Error, MessageCodec};
use std::{future::Future, sync::Arc};
use tokio::sync::Mutex;

use super::server::AttServer;
use crate::{AttributeBuilder, Error, SiCodec};

///
///
///
#[derive(Clone)]
pub struct SiAttServer {
    ///
    /// Inner server implementation
    pub inner: Arc<Mutex<AttServer<SiCodec>>>,

    unit: String,
    min: i32,
    max: i32,

    decimals: u32,
}

impl SiAttServer {
    ///
    ///
    pub fn r#type() -> String {
        "si".to_string()
    }

    ///
    ///
    ///
    pub fn new<N: Into<String>>(
        builder: AttributeBuilder,
        unit: N,
        min: i32,
        max: i32,
        decimals: u32,
    ) -> Self {
        Self {
            inner: Arc::new(Mutex::new(AttServer::<SiCodec>::from(builder))),
            unit: unit.into(),
            min: min,
            max: max,
            decimals: decimals,
        }
    }

    ///
    /// Bloc until at least a command is received
    ///
    pub async fn wait_commands(&self) {
        let in_notifier = self.inner.lock().await.in_notifier();
        in_notifier.notified().await
    }

    ///
    /// Bloc until at least a command is received then execute the 'function'
    ///
    pub async fn wait_commands_then<F>(&self, function: F) -> Result<(), Error>
    where
        F: Future<Output = Result<(), Error>> + Send + 'static,
    {
        let in_notifier = self.inner.lock().await.in_notifier();
        in_notifier.notified().await;
        function.await
    }

    ///
    /// Get the value of the attribute
    /// If None, the first value is not yet received
    ///
    pub async fn pop_cmd_as_f32(&mut self) -> Option<Result<f32, Error>> {
        self.inner
            .lock()
            .await
            .pop_cmd()
            .and_then(|v| Some(v.into_f32()))
    }

    ///
    /// Get the value of the attribute
    /// If None, the first value is not yet received
    ///
    pub async fn get_last_cmd_as_f32(&self) -> Option<Result<f32, Error>> {
        self.inner
            .lock()
            .await
            .get_last_cmd()
            .and_then(|v| Some(v.into_f32()))
    }

    /// Set the value of the attribute
    ///
    pub async fn set_from_f32(&self, value: f32) -> Result<(), Error> {
        self.inner
            .lock()
            .await
            .set(SiCodec::from_f32(value, self.decimals))
            .await?;
        Ok(())
    }

    pub async fn send_alert<T: Into<String>>(&self, message: T) {
        self.inner.lock().await.send_alert(message.into());
    }
}
