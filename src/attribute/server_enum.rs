// use super::AttOnlyMsgAttInner;
// use crate::{AttributeBuilder, Error, MessageCodec};
use std::{future::Future, sync::Arc};
use tokio::sync::Mutex;

use super::server::AttServer;
use crate::{AttributeBuilder, Error, StringCodec};

///
///
///
#[derive(Clone)]
pub struct EnumAttServer {
    ///
    /// Inner server implementation
    pub inner: Arc<Mutex<AttServer<StringCodec>>>,

    ///
    ///
    ///
    choices: Vec<String>,
}

impl EnumAttServer {
    ///
    ///
    pub fn r#type() -> String {
        "enum".to_string()
    }

    ///
    ///
    ///
    pub fn new(builder: AttributeBuilder, choices: Vec<String>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(AttServer::<StringCodec>::from(builder))),
            choices: choices,
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
    pub async fn pop_cmd(&mut self) -> Option<String> {
        self.inner
            .lock()
            .await
            .pop_cmd()
            .and_then(|v| Some(v.value))
    }

    ///
    /// Get the value of the attribute
    /// If None, the first value is not yet received
    ///
    pub async fn get_last_cmd(&self) -> Option<String> {
        return self
            .inner
            .lock()
            .await
            .get_last_cmd()
            .and_then(|v| Some(v.value));
    }

    /// Set the value of the attribute
    ///
    pub async fn set(&self, value: String) -> Result<(), Error> {
        self.inner
            .lock()
            .await
            .set(StringCodec { value: value })
            .await?;
        Ok(())
    }
}
