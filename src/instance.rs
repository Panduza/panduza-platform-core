mod inner;
use crate::InterfaceBuilder;
use crate::{
    reactor::Reactor, AttributeBuilder, DriverOperations, Error, InstanceLogger, InstanceSettings,
    Notification, TaskResult, TaskSender,
};
use futures::FutureExt;
pub use inner::InstanceInner;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, future::Future, sync::Arc};
use tokio::sync::Mutex;
use tokio::sync::{mpsc::Sender, Notify};
pub mod monitor;

use crate::log_error;

/// States of the main Interface FSM
///
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum State {
    Booting,
    Connecting,
    Initializating,
    Running,
    Warning,
    Error,
    Cleaning,
    Stopping,
    #[default]
    Undefined,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            State::Booting => write!(f, "Booting"),
            State::Connecting => write!(f, "Connecting"),
            State::Initializating => write!(f, "Initializating"),
            State::Running => write!(f, "Running"),
            State::Error => write!(f, "Error"),
            State::Warning => write!(f, "Warning"),
            State::Cleaning => write!(f, "Cleaning"),
            State::Stopping => write!(f, "Stopping"),
            State::Undefined => write!(f, "Undefined"),
        }
    }
}

///
///
///
#[derive(Clone)]
pub struct Instance {
    ///
    /// Logger for driver instance
    ///
    pub logger: InstanceLogger,

    ///
    /// Manage all MQTT communications
    ///
    reactor: Reactor,

    // ///
    // /// Device must share its status with the device "_" through this info object
    // info_dyn_dev_status: Option<Arc<Mutex<InfoDynamicDeviceStatus>>>,
    pub r_notifier: Option<Sender<Notification>>,

    // started: bool,
    /// Inner object
    inner: Arc<Mutex<InstanceInner>>,

    /// Operations of the devices
    ///
    inner_operations: Arc<Mutex<Box<dyn DriverOperations>>>,

    ///
    topic: String,

    // platform_services: crate::platform::services::AmServices,
    // // logger: Logger,
    state: Arc<Mutex<State>>,
    state_change_notifier: Arc<Notify>,
    //
    //
    spawner: TaskSender<Result<(), Error>>,
}

impl Instance {
    //
    // reactor

    /// Create a new instance of the Device
    ///
    pub fn new(
        reactor: Reactor,
        r_notifier: Option<Sender<Notification>>,
        spawner: TaskSender<Result<(), Error>>,
        name: String,
        operations: Box<dyn DriverOperations>,
        settings: Option<InstanceSettings>,
    ) -> Instance {
        // Create the object
        Instance {
            logger: InstanceLogger::new(name.clone()),
            reactor: reactor.clone(),
            // info_pack: info_pack,
            // info_dyn_dev_status: None,
            r_notifier: r_notifier,
            inner: InstanceInner::new(reactor.clone(), settings).into(),
            inner_operations: Arc::new(Mutex::new(operations)),
            topic: format!("{}/{}", reactor.root_topic(), name),
            state: Arc::new(Mutex::new(State::Booting)),
            state_change_notifier: Arc::new(Notify::new()),
            spawner: spawner,
        }
    }

    ///
    /// Set the plugin name inside the logger
    ///
    pub fn set_plugin<A: Into<String>>(&mut self, text: A) {
        self.logger.set_plugin(text);
    }

    /// Simple getter for Reactor
    ///
    pub fn reactor(&self) -> &Reactor {
        &self.reactor
    }

    pub async fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = TaskResult> + Send + 'static,
    {
        self.spawner.spawn(future.boxed()).unwrap();
    }

    ///
    /// Spawn a new task and attach a name to it into logs
    ///
    pub async fn spawn_with_name<N: Into<String>, F>(&mut self, name: N, future: F)
    where
        F: Future<Output = TaskResult> + Send + 'static,
    {
        self.spawner.spawn_with_name(name, future.boxed()).unwrap();
    }

    ///
    /// Create a new interface from this device
    ///
    pub fn create_class<N: Into<String>>(&mut self, name: N) -> InterfaceBuilder {
        InterfaceBuilder::new(
            self.reactor.clone(),
            self.clone(),
            // self.info_dyn_dev_status.clone(),
            format!("{}/{}", self.topic, name.into()), // take the device topic as root
        )
    }

    ///
    /// Device can directly create some attribute on its root
    ///
    pub fn create_attribute<N: Into<String>>(&mut self, name: N) -> AttributeBuilder {
        self.reactor
            .create_new_attribute(self.r_notifier.clone())
            .with_topic(format!("{}/{}", self.topic, name.into())) // take the device topic as root
    }

    // pub async fn run(&mut self) {}

    ///
    /// Run the FSM of the device
    ///
    pub async fn run_fsm(&mut self) {
        //
        // First start by booting the device to give him a connection with the info_pack
        // and allow the InfoDevice to send device information on MQTT
        self.move_to_state(State::Booting).await;

        //
        // Start the main loop of the device
        // TODO => Maybe we should give a way to stop properly this task instead of canceling the task brutally
        loop {
            self.state_change_notifier.notified().await;

            // Helper log
            let stateee = self.state.lock().await.clone();
            self.logger.debug(format!("FSM State {}", stateee));

            // Perform state task
            match stateee {
                State::Booting => {
                    // if let Some(mut info_pack) = self.info_pack.clone() {
                    //     self.logger.debug("FSM try to add_deivce in info pack");
                    //     self.info_dyn_dev_status = Some(info_pack.add_device(self.name()).await);
                    //     self.logger.debug("FSM finish info pack");
                    // } else {
                    //     self.logger.debug("FSM NO INFO PACK !");
                    // }
                    self.move_to_state(State::Initializating).await;
                }
                State::Connecting => {} // wait for reactor signal
                State::Initializating => {
                    //
                    // Try to mount the device
                    let mount_result = self.inner_operations.lock().await.mount(self.clone()).await;
                    //
                    // Manage mount result
                    match mount_result {
                        Ok(_) => {
                            self.logger.debug("FSM Mount Success ");
                            self.move_to_state(State::Running).await;
                        }
                        Err(e) => {
                            log_error!(self.logger, "Instance Mount Failure '{:?}'", e);
                            self.move_to_state(State::Error).await;
                        }
                    }
                }
                State::Running => {} // do nothing, watch for inner tasks
                State::Error => {
                    //
                    // Wait before reboot
                    self.inner_operations
                        .lock()
                        .await
                        .wait_reboot_event(self.clone())
                        .await;
                    self.logger.info("try to reboot");
                    self.move_to_state(State::Initializating).await;
                }
                State::Warning => {}
                State::Cleaning => {}
                State::Stopping => {}
                State::Undefined => {}
            }
        }

        // Ok(())
    }

    ///
    /// Clone settings of the device
    ///
    pub async fn settings(&self) -> Option<InstanceSettings> {
        self.inner.lock().await.settings.clone()
    }

    pub fn name(&self) -> String {
        match self.topic.split('/').last() {
            Some(value) => value.to_string(),
            None => "noname".to_string(),
        }
    }

    pub async fn go_error(&mut self) {
        // println!("GO ERROR");
        self.move_to_state(State::Error).await;
    }

    ///
    /// Function to change the current state of the device FSM
    ///
    pub async fn move_to_state(&mut self, new_state: State) {
        // Set the new state
        *self.state.lock().await = new_state.clone();

        // println!("new state !!! {:?}", new_state.clone());

        // Alert monitoring device "_"
        if let Some(r_notifier) = &mut self.r_notifier {
            r_notifier
                .try_send(Notification::new_state_changed_notification(
                    self.topic.clone(),
                    new_state.clone(),
                ))
                .unwrap();
        }
        // else {
        //     self.logger
        //         .debug("!!!!!!! DEBUG !!!!!!! r_notifier is 'None'");
        // }

        // Notify FSM
        self.state_change_notifier.notify_one();
    }
}
