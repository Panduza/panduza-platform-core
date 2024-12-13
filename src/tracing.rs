mod csv_formatter;
mod logger;
mod multi_writer;

pub use logger::AttributeLogger;
pub use logger::DriverLogger;
pub use logger::FactoryLogger;
pub use logger::InstanceLogger;
pub use logger::Logger;
pub use logger::PlatformLogger;
pub use logger::RuntimeLogger;

use csv_formatter::CSVFormatter;
use multi_writer::MultiWriter;

/// Function to initiliaze tracing for the application
///
pub fn init(enable_stdout: bool, enable_broker_log: bool, debug: bool, trace: bool) {
    //
    //
    let level = tracing::Level::TRACE;

    let subscriber = tracing_subscriber::fmt()
        // .with_max_level(tracing::Level::TRACE)
        .with_max_level(level)
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Build the subscriber
        .event_format(CSVFormatter {})
        // Custom writer
        .with_writer(move || MultiWriter::new(enable_stdout, enable_broker_log, debug, trace))
        // Ok
        .finish();
    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber).unwrap();
}
