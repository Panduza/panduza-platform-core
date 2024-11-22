use crate::{DeviceSettings, Reactor};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Inner implementation of the device
pub struct InstanceInner {
    ///
    ///
    pub reactor: Reactor,

    /// Settings of the device, provided by the user
    ///
    pub settings: Option<DeviceSettings>,
}

impl InstanceInner {
    pub fn new(reactor: Reactor, settings: Option<DeviceSettings>) -> InstanceInner {
        InstanceInner {
            reactor: reactor,
            settings: settings,
        }
    }
}

/// Allow mutation into Arc pointer
impl Into<Arc<Mutex<InstanceInner>>> for InstanceInner {
    fn into(self) -> Arc<Mutex<InstanceInner>> {
        Arc::new(Mutex::new(self))
    }
}
