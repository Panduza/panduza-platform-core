use super::{class_builder::ClassBuilder, Container};
use crate::{AttributeBuilder, Logger, TaskResult};
use crate::{Instance, Reactor};
use async_trait::async_trait;
use std::future::Future;

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
            self.instance.reactor.clone(),
            self.instance.clone(),
            // self.device_dyn_info.clone(),
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
        }
    }
}
