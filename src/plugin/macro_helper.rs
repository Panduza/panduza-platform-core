#[macro_export]
macro_rules! plugin_interface {
    () => {
        use std::{
            sync::{Arc, Mutex},
            thread::{self, JoinHandle},
            time::Duration,
        };

        use panduza_platform_core::{
            Factory, Plugin, Producer, ProductionOrder, Reactor, ReactorSettings, Runtime,
        };
        use serde_json::Result;
        use serde_json::Value;
        use tokio::time::sleep;

        ///
        /// True when the runtime has been initialized
        ///
        static mut RUNTIME_STARTED: bool = false;

        static mut FACTORY: Option<Arc<Mutex<Factory>>> = None;

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
            panduza_platform_core::log::init();

            //
            //
            let mut factory = Factory::new();
            factory.add_producers(plugin_producers());

            //
            let settings = ReactorSettings::new("localhost", 1883, None);
            let mut reactor = Reactor::new(settings);

            //
            //
            let mut runtime = Runtime::new(factory, reactor);
            runtime.set_plugin("pza-plugin-fakes");

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

        pub extern "C" fn pok() {
            println!("pooook");

            unsafe {
                start_runtime();
            }

            // handle.join().unwrap();
        }

        pub unsafe extern "C" fn join() {
            THREAD_HANDLE.take().unwrap().join().unwrap();
        }

        pub unsafe extern "C" fn produce(str_production_order: *const i8) -> u32 {
            //
            // Start runtime if not already
            start_runtime();

            let po = ProductionOrder::from_c_str_ptr(str_production_order).unwrap();
            println!("{:?}", po);

            POS.as_mut().unwrap().try_send(po).unwrap();

            // Success
            0
        }

        #[no_mangle]
        pub unsafe extern "C" fn plugin_entry_point() -> Plugin {
            // if factory none
            // init factory
            let mut factory = Factory::new();
            factory.add_producers(plugin_producers());
            unsafe {
                FACTORY = Some(Arc::new(Mutex::new(factory)));
            }

            // if reactor none
            // init reactor

            // build runtine

            let p = Plugin::new(c"tok", c"v0.1", pok, join, produce);

            // println!("pp {:?}", *(p.name) as u8);

            return p;
        }
    };
}
