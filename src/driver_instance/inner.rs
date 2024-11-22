use crate::{DeviceSettings, Reactor};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Inner implementation of the device
pub struct DriverInstanceInner {
    ///
    ///
    pub reactor: Reactor,

    /// Settings of the device, provided by the user
    ///
    pub settings: Option<DeviceSettings>,
}

impl DriverInstanceInner {
    pub fn new(reactor: Reactor, settings: Option<DeviceSettings>) -> DriverInstanceInner {
        DriverInstanceInner {
            reactor: reactor,
            settings: settings,
        }
    }
}

/// Allow mutation into Arc pointer
impl Into<Arc<Mutex<DriverInstanceInner>>> for DriverInstanceInner {
    fn into(self) -> Arc<Mutex<DriverInstanceInner>> {
        Arc::new(Mutex::new(self))
    }
}
