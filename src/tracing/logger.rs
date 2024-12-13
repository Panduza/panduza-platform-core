use crate::Topic;

/// Generic way to build logs on the platform
///
#[derive(Clone)]
pub struct Logger {
    pub class: String,
    pub i1: String,
    pub i2: String,
    pub i3: String,
    pub plugin: String,
}
impl Logger {
    /// Create a new logger
    ///
    pub fn new<A: Into<String>, B: Into<String>, C: Into<String>, D: Into<String>>(
        class: A,
        i1: B,
        i2: C,
        i3: D,
    ) -> Logger {
        return Logger {
            class: class.into(),
            i1: i1.into(),
            i2: i2.into(),
            i3: i3.into(),
            plugin: String::new(),
        };
    }

    /// Create a logger configured for attribute from its topic
    ///
    pub fn new_for_attribute_from_topic<A: Into<String>>(topic: A) -> Self {
        let topic_obj = Topic::from_string(topic.into());
        Self::new(
            "Attribute",
            topic_obj.instance_name(),
            "",
            topic_obj.leaf_name().unwrap_or(&"".to_string()),
        )
    }

    pub fn error<A: Into<String>>(&self, text: A) {
        tracing::error!(
            class = self.class,
            i1 = self.i1,
            i2 = self.i2,
            i3 = self.i3,
            plugin = self.plugin,
            "{}",
            text.into()
        );
    }

    pub fn warn<A: Into<String>>(&self, text: A) {
        tracing::warn!(
            class = self.class,
            i1 = self.i1,
            i2 = self.i2,
            i3 = self.i3,
            plugin = self.plugin,
            "{}",
            text.into()
        );
    }

    pub fn info<A: Into<String>>(&self, text: A) {
        tracing::info!(
            class = self.class,
            i1 = self.i1,
            i2 = self.i2,
            i3 = self.i3,
            plugin = self.plugin,
            "{}",
            text.into()
        );
    }

    pub fn debug<A: Into<String>>(&self, text: A) {
        tracing::debug!(
            class = self.class,
            i1 = self.i1,
            i2 = self.i2,
            i3 = self.i3,
            plugin = self.plugin,
            "{}",
            text.into()
        );
    }

    pub fn trace<A: Into<String>>(&self, text: A) {
        tracing::trace!(
            class = self.class,
            i1 = self.i1,
            i2 = self.i2,
            i3 = self.i3,
            plugin = self.plugin,
            "{}",
            text.into()
        );
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Clone)]
pub struct RuntimeLogger {
    base: Logger,
}
impl RuntimeLogger {
    pub fn new() -> RuntimeLogger {
        RuntimeLogger {
            base: Logger::new("Runtime", "", "", ""),
        }
    }
    pub fn error<A: Into<String>>(&self, text: A) {
        self.base.error(text);
    }
    pub fn warn<A: Into<String>>(&self, text: A) {
        self.base.warn(text);
    }
    pub fn info<A: Into<String>>(&self, text: A) {
        self.base.info(text);
    }
    pub fn debug<A: Into<String>>(&self, text: A) {
        self.base.debug(text);
    }
    pub fn trace<A: Into<String>>(&self, text: A) {
        self.base.trace(text);
    }
    pub fn set_plugin<A: Into<String>>(&mut self, text: A) {
        self.base.plugin = text.into();
    }
    pub fn get_plugin(&self) -> String {
        self.base.plugin.clone()
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Clone)]
pub struct PlatformLogger {
    base: Logger,
}
impl PlatformLogger {
    pub fn new() -> PlatformLogger {
        PlatformLogger {
            base: Logger::new("Platform", "", "", ""),
        }
    }
    pub fn error<A: Into<String>>(&self, text: A) {
        self.base.error(text);
    }
    pub fn warn<A: Into<String>>(&self, text: A) {
        self.base.warn(text);
    }
    pub fn info<A: Into<String>>(&self, text: A) {
        self.base.info(text);
    }
    pub fn debug<A: Into<String>>(&self, text: A) {
        self.base.debug(text);
    }
    pub fn trace<A: Into<String>>(&self, text: A) {
        self.base.trace(text);
    }
    pub fn set_plugin<A: Into<String>>(&mut self, text: A) {
        self.base.plugin = text.into();
    }
    pub fn get_plugin(&self) -> String {
        self.base.plugin.clone()
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Clone)]
pub struct FactoryLogger {
    base: Logger,
}
impl FactoryLogger {
    pub fn new() -> FactoryLogger {
        FactoryLogger {
            base: Logger::new("Factory", "", "", ""),
        }
    }
    pub fn info<A: Into<String>>(&self, text: A) {
        self.base.info(text);
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Clone)]
pub struct InstanceLogger {
    base: Logger,
}
impl InstanceLogger {
    pub fn new<A: Into<String>>(name: A) -> InstanceLogger {
        InstanceLogger {
            base: Logger::new("Instance", name.into(), "", ""),
        }
    }
    pub fn error<A: Into<String>>(&self, text: A) {
        self.base.error(text);
    }
    pub fn warn<A: Into<String>>(&self, text: A) {
        self.base.warn(text);
    }
    pub fn info<A: Into<String>>(&self, text: A) {
        self.base.info(text);
    }
    pub fn debug<A: Into<String>>(&self, text: A) {
        self.base.debug(text);
    }
    pub fn trace<A: Into<String>>(&self, text: A) {
        self.base.trace(text);
    }
    pub fn set_plugin<A: Into<String>>(&mut self, text: A) {
        self.base.plugin = text.into();
    }
    pub fn new_attribute_logger<A: Into<String>, B: Into<String>>(
        &self,
        classes: A,
        name: B,
    ) -> AttributeLogger {
        AttributeLogger::new(self.base.i1.clone(), classes, name)
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Clone)]
pub struct AttributeLogger {
    base: Logger,
}
impl AttributeLogger {
    pub fn new<A: Into<String>, B: Into<String>, C: Into<String>>(
        instance: A,
        classes: B,
        name: C,
    ) -> Self {
        Self {
            base: Logger::new("Attribute", instance.into(), classes.into(), name.into()),
        }
    }
    pub fn error<A: Into<String>>(&self, text: A) {
        self.base.error(text);
    }
    pub fn warn<A: Into<String>>(&self, text: A) {
        self.base.warn(text);
    }
    pub fn info<A: Into<String>>(&self, text: A) {
        self.base.info(text);
    }
    pub fn debug<A: Into<String>>(&self, text: A) {
        self.base.debug(text);
    }
    pub fn trace<A: Into<String>>(&self, text: A) {
        self.base.trace(text);
    }
    pub fn set_plugin<A: Into<String>>(&mut self, text: A) {
        self.base.plugin = text.into();
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Clone)]
pub struct DriverLogger {
    base: Logger,
}
impl DriverLogger {
    pub fn new<B: Into<String>, C: Into<String>, D: Into<String>>(i1: B, i2: C, i3: D) -> Self {
        Self {
            base: Logger::new("Driver", i1.into(), i2.into(), i3.into()),
        }
    }
    pub fn error<A: Into<String>>(&self, text: A) {
        self.base.error(text);
    }
    pub fn warn<A: Into<String>>(&self, text: A) {
        self.base.warn(text);
    }
    pub fn info<A: Into<String>>(&self, text: A) {
        self.base.info(text);
    }
    pub fn debug<A: Into<String>>(&self, text: A) {
        self.base.debug(text);
    }
    pub fn trace<A: Into<String>>(&self, text: A) {
        self.base.trace(text);
    }
    pub fn set_plugin<A: Into<String>>(&mut self, text: A) {
        self.base.plugin = text.into();
    }
}

#[macro_export]
macro_rules! log_error {
    ($logger:expr , $($arg:tt)*) => {
        $logger.error(format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_warn {
    ($logger:expr , $($arg:tt)*) => {
        $logger.warn(format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_info {
    ($logger:expr , $($arg:tt)*) => {
        $logger.info(format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_debug {
    ($logger:expr , $($arg:tt)*) => {
        $logger.debug(format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_trace {
    ($logger:expr , $($arg:tt)*) => {
        $logger.trace(format!($($arg)*))
    };
}
