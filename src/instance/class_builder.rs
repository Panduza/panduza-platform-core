use crate::{Class, ClassNotification, Reactor};

use super::Instance;

pub struct ClassBuilder {
    /// Parent class if any
    ///
    parent_class: Option<Class>,

    //
    pub reactor: Reactor,
    ///
    pub device: Instance,
    ///
    /// Option because '_' device will not provide one
    ///
    // pub device_dyn_info: Option<ThreadSafeInfoDynamicDeviceStatus>,
    // pub r_notifier: Option<Sender<Notification>>,
    ///
    pub topic: String,

    pub tags: Vec<String>,
}

impl ClassBuilder {
    pub fn new<N: Into<String>>(
        parent_class: Option<Class>,
        reactor: Reactor, // deprecated because acces through device
        device: Instance,
        // device_dyn_info: Option<ThreadSafeInfoDynamicDeviceStatus>,
        topic: N,
    ) -> Self {
        Self {
            parent_class: parent_class,
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
    pub fn finish(self) -> Class {
        let bis = self.topic.clone();
        if let Some(r_notifier) = self.device.r_notifier.clone() {
            r_notifier
                .try_send(ClassNotification::new(bis, self.tags.clone()).into())
                .unwrap();
        }
        // insert in status
        Class::from(self)
    }
}
