#[macro_export]
macro_rules! plugin_interface {
    () => {
        use std::ffi::CString;
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

        static mut FACTORY: Option<Factory> = None;

        static mut FACTORY_PRODUCER_REFS: Option<CString> = None;

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
            let factory = FACTORY.take();

            //
            let settings = ReactorSettings::new("localhost", 1883, None);
            let mut reactor = Reactor::new(settings);

            //
            //
            let mut runtime = Runtime::new(factory.unwrap(), reactor);
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

        pub unsafe extern "C" fn producer_refs() -> *const i8 {
            println!("{:?}", FACTORY_PRODUCER_REFS);
            FACTORY_PRODUCER_REFS.as_ref().unwrap().as_c_str().as_ptr()
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
                println!("{:?}", factory.producer_refs());
                FACTORY_PRODUCER_REFS = Some(factory.producer_refs_as_c_string().unwrap());
                println!("{:?}", FACTORY_PRODUCER_REFS);
                FACTORY = Some(factory);
            }

            // if reactor none
            // init reactor

            // build runtine

            let p = Plugin::new(c"tok", c"v0.1", pok, join, producer_refs, produce);

            // println!("pp {:?}", *(p.name) as u8);

            return p;
        }
    };
}
