// use super::AttOnlyMsgAttInner;
// use crate::{AttributeBuilder, Error, MessageCodec};
use std::{future::Future, sync::Arc};
use tokio::sync::Mutex;

use super::server::AttServer;
use crate::{generic_att_server_methods, AttributeBuilder, Error, StringCodec};

///
///
///
#[derive(Clone)]
pub struct StringAttServer {
    ///
    /// Inner server implementation
    pub inner: Arc<Mutex<AttServer<StringCodec>>>,
}

impl StringAttServer {
    ///
    ///
    pub fn r#type() -> String {
        "string".to_string()
    }

    ///
    ///
    ///
    pub fn new(builder: AttributeBuilder) -> Self {
        Self {
            inner: Arc::new(Mutex::new(AttServer::<StringCodec>::from(builder))),
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

    ///
    ///
    pub async fn send_alert<T: Into<String>>(&self, message: T) {
        self.inner.lock().await.send_alert(message.into());
    }

    generic_att_server_methods!();
}
