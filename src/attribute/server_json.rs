// use super::AttOnlyMsgAttInner;
// use crate::{AttributeBuilder, Error, MessageCodec};
use std::{future::Future, sync::Arc};
use tokio::sync::Mutex;

use super::server::AttServer;
use crate::{AttributeBuilder, Error, JsonCodec};

///
///
///
#[derive(Clone)]
pub struct JsonAttServer {
    ///
    /// Inner server implementation
    pub inner: Arc<Mutex<AttServer<JsonCodec>>>,
}

impl JsonAttServer {
    ///
    ///
    pub fn r#type() -> String {
        "json".to_string()
    }

    ///
    ///
    ///
    pub fn new(builder: AttributeBuilder) -> Self {
        Self {
            inner: Arc::new(Mutex::new(AttServer::<JsonCodec>::from(builder))),
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
    pub async fn pop_cmd(&mut self) -> Option<serde_json::Value> {
        self.inner
            .lock()
            .await
            .pop_cmd()
            .and_then(|v| Some(v.value))
    }

    /// Set the value of the attribute
    ///
    pub async fn set(&self, value: serde_json::Value) -> Result<(), Error> {
        self.inner
            .lock()
            .await
            .set(JsonCodec { value: value })
            .await?;
        Ok(())
    }

    ///
    ///
    pub async fn send_alert<T: Into<String>>(&self, message: T) {
        self.inner.lock().await.send_alert(message.into());
    }
}
