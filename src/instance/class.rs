use super::element::Element;
use super::{class_builder::ClassBuilder, Container};
use crate::{AttributeBuilder, Error, Instance, Logger, TaskResult};
use async_trait::async_trait;
use futures::lock::Mutex;
use std::future::Future;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Clone)]
///
///
///
pub struct Class {
    /// Class Logger
    ///
    logger: Logger,

    ///
    ///
    instance: Instance,

    ///
    ///
    topic: String,

    ///
    ///
    enabled: Arc<AtomicBool>,

    /// Sub elements
    ///
    sub_elements: Arc<Mutex<Vec<Element>>>,
}

impl Class {
    async fn change_enablement(&mut self, enabled: bool) -> Result<(), Error> {
        self.enabled.store(enabled, Ordering::Relaxed);
        Ok(())
    }
}

#[async_trait]
impl Container for Class {
    /// Get for the container logger
    ///
    fn logger(&self) -> &Logger {
        &self.logger
    }

    /// Override
    ///
    fn create_class<N: Into<String>>(&mut self, name: N) -> ClassBuilder {
        ClassBuilder::new(
            Some(self.clone()),
            self.instance.reactor.clone(),
            self.instance.clone(),
            format!("{}/{}", self.topic, name.into()), // take the device topic as root
        )
    }

    /// Override
    ///
    fn create_attribute<N: Into<String>>(&mut self, name: N) -> AttributeBuilder {
        self.instance
            .reactor
            .create_new_attribute(self.instance.r_notifier.clone())
            .with_topic(format!("{}/{}", self.topic, name.into()))
    }

    /// Override
    ///
    async fn spawn<N: Send + Into<String>, F>(&mut self, name: N, future: F)
    where
        F: Future<Output = TaskResult> + Send + 'static,
    {
        self.instance.spawn(name, future).await;
    }
}

impl From<ClassBuilder> for Class {
    fn from(builder: ClassBuilder) -> Self {
        Class {
            logger: builder.device.logger.new_for_class(&builder.topic),
            instance: builder.device,
            topic: builder.topic,
            enabled: Arc::new(AtomicBool::new(true)),
            sub_elements: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
