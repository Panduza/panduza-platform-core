use super::Interface;
use crate::Device;
use crate::Notification;
use crate::Reactor;

pub struct InterfaceBuilder {
    //
    pub reactor: Reactor,
    ///
    pub device: Device,
    ///
    /// Option because '_' device will not provide one
    ///
    // pub device_dyn_info: Option<ThreadSafeInfoDynamicDeviceStatus>,
    // pub r_notifier: Option<Sender<Notification>>,
    ///
    pub topic: String,

    pub tags: Vec<String>,
}

impl InterfaceBuilder {
    pub fn new<N: Into<String>>(
        reactor: Reactor, // deprecated because acces through device
        device: Device,
        // device_dyn_info: Option<ThreadSafeInfoDynamicDeviceStatus>,
        topic: N,
    ) -> Self {
        Self {
            reactor: reactor,
            device: device,
            // device_dyn_info: device_dyn_info,
            topic: topic.into(),
            tags: Vec::new(),
        }
    }

    pub fn with_tag<T: Into<String>>(mut self, tag: T) -> Self {
        self.tags.push(tag.into());
        self
    }

    ///
    ///
    ///
    pub fn finish(self) -> Interface {
        let bis = self.topic.clone();
        if let Some(r_notifier) = self.device.r_notifier.clone() {
            r_notifier
                .try_send(Notification::new_interface_element_created_notification(
                    bis,
                    self.tags.clone(),
                ))
                .unwrap();
        }
        // insert in status
        Interface::from(self)
    }
}
