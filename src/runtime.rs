use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use tokio::{sync::Mutex, task::JoinSet};
use crate::{TaskReceiver, TaskResult, TaskSender};





///
/// Provide a way to run and managed a pack of devices
/// 
struct Runtime {
    // pointer to a factory
    // pointer to a reactor


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
}


// async task for the runtime



impl Runtime {


    pub async fn task(mut self) -> TaskResult {
        
        // Remove receiver from self
        let mut task_receiver = self.task_receiver.take().ok_or(
            crate::Error::InternalLogic("Object 'task_receiver' is 'None'".to_string())
        )?;

        //
        while self.keep_alive.load(Ordering::Relaxed) {
            tokio::select! {
                device_task = task_receiver.rx.recv() => {
                    // Function to effectily spawn tasks requested by the system
                    let ah = self.task_pool.spawn(device_task.unwrap());
                    // self.logger.debug(format!( "New device task created ! [{:?}]", ah ));
                },
                _ = self.end_of_all_tasks() => {
                    // self.logger.warn("All tasks completed, stop the platform");
                    break;
                }
            }
        }


        Ok(())
    }


    /// Wait for all tasks to complete
    ///
    async fn end_of_all_tasks(&mut self) {

        // tokio::select! {
        //     a = self.task_pool.join_next() => {
        //         if a.is_some() {       
        //             match a.unwrap() {
        //                 Ok(a) => match a {
        //                     Ok(_) => {
        //                         self.logger.warn("main Task completed");
        //                     }
        //                     Err(e) => {
        //                         self.logger.error(format!("main Task failed: {}", e));
        //                         self.main_task_pool.abort_all();
        //                     }
        //                 },
        //                 Err(e) => {
        //                     self.logger.error(format!("main Join failed: {}", e));
        //                 }
        //             }
        //         }
        //         else {
        //             println!("main none join handle")
        //         }
        //     }}
        // }
    }
}
