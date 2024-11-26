// use super::AttOnlyMsgAttInner;
// use crate::{AttributeBuilder, Error, MessageCodec};
use std::{future::Future, sync::Arc};
use tokio::sync::Mutex;

use super::server::AttServer;
use crate::{AttributeBuilder, Error, NumberCodec};

///
///
///
#[derive(Clone)]
pub struct NumberAttServer {
    ///
    /// Inner server implementation
    pub inner: Arc<Mutex<AttServer<NumberCodec>>>,
}

impl NumberAttServer {
    ///
    ///
    pub fn r#type() -> String {
        "number".to_string()
    }

    ///
    ///
    ///
    pub fn new(builder: AttributeBuilder) -> Self {
        Self {
            inner: Arc::new(Mutex::new(AttServer::<NumberCodec>::from(builder))),
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
    pub async fn pop_cmd_as_i64(&mut self) -> Option<i64> {
        self.inner
            .lock()
            .await
            .pop_cmd()
            .and_then(|v| v.value.as_i64())
    }

    /// Set the value of the attribute
    ///
    pub async fn set_from_i64(&self, value: i64) -> Result<(), Error> {
        self.inner.lock().await.set(value.into()).await?;
        Ok(())
    }

    pub async fn send_alert<T: Into<String>>(&self, message: T) {
        self.inner.lock().await.send_alert(message.into());
    }
}
