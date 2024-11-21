#[macro_export]
macro_rules! plugin_interface {
    ($plg_name:literal) => {
        use panduza_platform_core::NotificationGroup;
        use panduza_platform_core::{
            Factory, PlatformLogger, Plugin, Producer, ProductionOrder, Reactor, ReactorSettings,
            Runtime, Scanner,
        };
        use serde_json::Result;
        use serde_json::Value;
        use std::ffi::CString;
        use std::{
            sync::{Arc, Mutex},
            thread::{self, JoinHandle},
            time::Duration,
        };
        use tokio::time::sleep;

        ///
        /// True when the runtime has been initialized
        ///
        static mut RUNTIME_STARTED: bool = false;

        ///
        ///
        ///
        static mut LOGGER: Option<PlatformLogger> = None;

        ///
        ///
        ///
        static mut PLG_NAME: Option<CString> = None;

        ///
        ///
        ///
        static mut RUNTIME_NOTIFICATIONS_GROUP: Option<Arc<std::sync::Mutex<NotificationGroup>>> =
            None;

        static mut FACTORY: Option<Factory> = None;

        static mut FACTORY_STORE: Option<CString> = None;

        static mut FACTORY_SCAN_RESULT: Option<CString> = None;

        static mut THREAD_HANDLE: Option<JoinHandle<()>> = None;

        static mut POS: Option<tokio::sync::mpsc::Sender<ProductionOrder>> = None;

        #[tokio::main]
        async fn start_async_runtime(runtime: Runtime) {
            runtime.task().await.unwrap();
        }

        ///
        /// Start the runtime
        ///
        unsafe fn start_runtime() {
            //
            // Already started
            if RUNTIME_STARTED {
                return;
            }

            //
            //
            let factory = FACTORY.take();

            //
            let settings = ReactorSettings::new("localhost", 1883, None);
            let mut reactor = Reactor::new(settings);

            //
            //
            let mut runtime = Runtime::new(factory.unwrap(), reactor);
            runtime.set_plugin($plg_name);
            RUNTIME_NOTIFICATIONS_GROUP = Some(runtime.clone_notifications());

            //
            //
            POS = Some(runtime.clone_production_order_sender());

            //
            // Start thread
            let __handle: JoinHandle<()> = thread::spawn(move || {
                start_async_runtime(runtime);
            });
            THREAD_HANDLE = Some(__handle);

            //
            // Set flag
            RUNTIME_STARTED = true;
        }

        pub unsafe extern "C" fn join() {
            THREAD_HANDLE.take().unwrap().join().unwrap();
        }

        pub unsafe extern "C" fn store() -> *const i8 {
            LOGGER.as_ref().unwrap().trace(format!("store !"));
            FACTORY_STORE.as_ref().unwrap().as_c_str().as_ptr()
        }

        pub unsafe extern "C" fn scan() -> *const i8 {
            LOGGER.as_ref().unwrap().trace(format!("scan !"));

            //
            // Start scan
            unsafe {
                FACTORY_SCAN_RESULT = Some(FACTORY.as_ref().unwrap().scan_as_c_string().unwrap());
            }

            //
            // Put the result available to the platform
            FACTORY_SCAN_RESULT.as_ref().unwrap().as_c_str().as_ptr()
        }

        pub unsafe extern "C" fn produce(str_production_order: *const i8) -> u32 {
            LOGGER.as_ref().unwrap().trace("produce");

            //
            // Start runtime if not already
            start_runtime();

            let po = ProductionOrder::from_c_str_ptr(str_production_order).unwrap();
            POS.as_mut().unwrap().try_send(po).unwrap();

            // Success
            0
        }

        ///
        /// Pull notifications from the runtime
        ///
        pub unsafe extern "C" fn pull_notifications() -> *const i8 {
            //
            // Debug log
            // LOGGER.as_ref().unwrap().debug("pull_notifications");

            //
            // Pull notifications from the runtime
            match &RUNTIME_NOTIFICATIONS_GROUP {
                Some(notifications) => {
                    return notifications.lock().unwrap().pull_and_serialize();
                }
                None => {
                    LOGGER
                        .as_ref()
                        .unwrap()
                        .error("RUNTIME_NOTIFICATIONS_GROUP is 'None'");
                    return std::ptr::null();
                }
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn plugin_entry_point() -> Plugin {
            //
            // Create a static reference for the plugin name
            // in order to provide a static pointer to the main program
            PLG_NAME = Some(CString::new($plg_name).unwrap());

            //
            // Init logging system on the plugin
            panduza_platform_core::log::init();
            let mut logger = PlatformLogger::new();
            logger.set_plugin($plg_name);
            logger.info("plugin_entry_point");
            LOGGER = Some(logger);

            // if factory none
            // init factory
            let mut factory = Factory::new();
            factory.add_producers(plugin_producers());
            factory.add_scanners(plugin_scanners());
            unsafe {
                FACTORY_STORE = Some(factory.store_as_c_string().unwrap());
                FACTORY = Some(factory);
            }

            //
            // Start runtime
            start_runtime();

            //
            //
            let p = Plugin::new(
                PLG_NAME.as_ref().unwrap().as_c_str(),
                c"v0.1",
                join,
                store,
                scan,
                produce,
                pull_notifications,
            );
            return p;
        }
    };
}
