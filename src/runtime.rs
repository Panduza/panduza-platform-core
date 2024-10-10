use crate::{
    task_channel::create_task_channel, Factory, ProductionOrder, Reactor, RuntimeLogger,
    TaskReceiver, TaskResult, TaskSender,
};
use futures::FutureExt;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::sync::mpsc::channel;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
use tokio::task::JoinSet;

///
///
///
static TASK_CHANNEL_SIZE: usize = 64;

///
///
///
static PROD_ORDER_CHANNEL_SIZE: usize = 64;

///
/// Provide a way to run and managed a pack of devices
///
pub struct Runtime {
    ///
    /// Logger dedicated to runtime activity
    ///
    logger: RuntimeLogger,
    ///
    ///
    factory: Factory,
    ///
    ///
    reactor: Reactor,
    ///
    ///
    keep_alive: Arc<AtomicBool>,
    ///
    /// Pool
    task_pool: JoinSet<TaskResult>,
    ///
    /// Sender, allow a sub function to start a task managed by this runtime
    task_sender: TaskSender<TaskResult>,
    ///
    /// Receiver, catch task request and start them inside this runtime
    task_receiver: Option<TaskReceiver<TaskResult>>,
    ///
    /// Sender, allow a sub function to request a register a production order
    production_order_sender: Sender<ProductionOrder>,
    /// Sender, allow a sub function to request a register a production order
    production_order_receiver: Option<Receiver<ProductionOrder>>,
}

impl Runtime {
    ///
    /// Constructor
    ///
    pub fn new(factory: Factory, reactor: Reactor) -> Self {
        let (t_tx, t_rx) = create_task_channel::<TaskResult>(TASK_CHANNEL_SIZE);
        let (po_tx, po_rx) = channel::<ProductionOrder>(PROD_ORDER_CHANNEL_SIZE);
        Self {
            logger: RuntimeLogger::new(),
            factory: factory,
            reactor: reactor,
            keep_alive: Arc::new(AtomicBool::new(true)),
            task_pool: JoinSet::new(),
            task_sender: t_tx.clone(),
            task_receiver: Some(t_rx),
            production_order_sender: po_tx.clone(),
            production_order_receiver: Some(po_rx),
        }
    }

    ///
    /// Set the plugin name inside the logger
    ///
    pub fn set_plugin<A: Into<String>>(&mut self, text: A) {
        self.logger.set_plugin(text);
    }

    ///
    /// Getter for 'task_sender', need to be get before task start
    ///
    pub fn clone_task_sender(&self) -> TaskSender<TaskResult> {
        self.task_sender.clone()
    }

    ///
    /// Getter for 'production_order_sender', need to be get before task start
    ///
    pub fn clone_production_order_sender(&self) -> Sender<ProductionOrder> {
        self.production_order_sender.clone()
    }

    ///
    /// Main task of the runtime, it consume the object itself
    ///
    pub async fn task(mut self) -> TaskResult {
        //
        // Debug log
        self.logger.info("Runtime started !");

        self.reactor.start(self.task_sender.clone()).unwrap();

        //
        // Remove task receiver from self
        let mut task_receiver = self
            .task_receiver
            .take()
            .ok_or(crate::Error::InternalLogic(
                "Object 'task_receiver' is 'None'".to_string(),
            ))?;

        //
        // Remove production order receiver from self
        let mut p_order_receiver =
            self.production_order_receiver
                .take()
                .ok_or(crate::Error::InternalLogic(
                    "Object 'production_order_receiver' is 'None'".to_string(),
                ))?;

        //
        while self.keep_alive.load(Ordering::Relaxed) {
            tokio::select! {
                task = task_receiver.rx.recv() => {
                    // Function to effectily spawn tasks requested by the system
                    let ah = self.task_pool.spawn(task.unwrap());
                    self.logger.info(format!( "New device task created ! [{:?}]", ah ));
                },
                production_order = p_order_receiver.recv() => {
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    self.logger.debug(format!( "PROD REQUEST ! [{:?}]", production_order ));

                    // let mut production_order = ProductionOrder::new("panduza.picoha-dio", "testdevice");
                    // production_order.device_settings = json!({});
                    let (mut monitor, mut dev) =
                        self.factory
                            .produce(self.reactor.clone(), None, production_order.unwrap());

                    dev.set_plugin(self.logger.get_plugin());

                    // let mut dddddd2 = dev.clone();
                    self.task_sender
                        .spawn(
                            async move {
                                dev.run_fsm().await;
                                Ok(())
                            }
                            .boxed(),
                        )
                        .unwrap();

                    self.task_sender
                        .spawn(
                            async move {
                                monitor.run().await;
                                Ok(())
                            }
                            .boxed(),
                        )
                        .unwrap();

                },
                _ = self.end_of_all_tasks() => {
                    self.logger.warn("All tasks completed");
                    tokio::time::sleep(Duration::from_secs(2)).await;
                }
            }
        }

        //
        // Debug log
        self.logger.warn("Runtime over !");

        //
        // Return ok
        Ok(())
    }

    /// Wait for all tasks to complete
    ///
    async fn end_of_all_tasks(&mut self) {
        //
        // Make tasks run
        while let Some(join_result) = self.task_pool.join_next().await {
            match join_result {
                Ok(jr) => match jr {
                    Ok(_) => {
                        self.logger.warn("Task completed successly");
                    }
                    Err(e) => {
                        self.logger.error(format!("Task end badly: {:?}", e));
                        self.task_pool.abort_all();
                    }
                },
                Err(e) => {
                    self.logger.error(format!("Task join_next error: {:?}", e));
                }
            }
        }
    }
}
