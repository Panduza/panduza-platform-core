use std::{future::Future, sync::Arc};
use tokio::sync::Mutex;

use super::server::AttServer;
use crate::{generic_att_server_methods, AttributeBuilder, Error, Logger, SiCodec};

///
///
///
#[derive(Clone)]
pub struct SiAttServer {
    /// Local logger
    ///
    logger: Logger,

    ///
    /// Inner server implementation
    pub inner: Arc<Mutex<AttServer<SiCodec>>>,

    _unit: String,
    _min: i32,
    _max: i32,

    decimals: u32,
}

impl SiAttServer {
    //
    // Require inner member
    generic_att_server_methods!();

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
        let obj = AttServer::<SiCodec>::from(builder);
        Self {
            logger: obj.logger.clone(),
            inner: Arc::new(Mutex::new(obj)),
            _unit: unit.into(),
            _min: min,
            _max: max,
            decimals: decimals,
        }
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
}