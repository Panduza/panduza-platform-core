mod csv_formatter;
mod multi_writer;

use csv_formatter::CSVFormatter;
use multi_writer::MultiWriter;

/// Function to initiliaze tracing for the application
///
pub fn init(enable_stdout: bool, enable_broker_log: bool) {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Build the subscriber
        .event_format(CSVFormatter {})
        // Custom writer
        .with_writer(move || MultiWriter::new(enable_stdout, enable_broker_log))
        // Ok
        .finish();
    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber).unwrap();
}
