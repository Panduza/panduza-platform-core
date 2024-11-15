// use super::AttOnlyMsgAttInner;
// use crate::{AttributeBuilder, Error, MessageCodec};
use std::sync::Arc;
use tokio::sync::Mutex;

use super::server::AttServer;
use crate::SiCodec;

///
///
///
#[derive(Clone)]
pub struct SiAttServer {
    ///
    /// Inner server implementation
    inner: Arc<Mutex<AttServer<SiCodec>>>,
}
