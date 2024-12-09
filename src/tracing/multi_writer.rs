mod line_printer;

use crate::env::system_default_log_dir;

use line_printer::print_log_line;
use tracing_appender::rolling::RollingFileAppender;
use tracing_appender::rolling::Rotation;

///
///
///
pub struct MultiWriter {
    enable_stdout: bool,
    enable_broker_log: bool,
    debug: bool,
    trace: bool,
    filea: tracing_appender::rolling::RollingFileAppender,
}

// tracing_appender::rolling::never(".", "platform-log.csv")
impl MultiWriter {
    pub fn new(enable_stdout: bool, enable_broker_log: bool, debug: bool, trace: bool) -> Self {
        MultiWriter {
            enable_stdout: enable_stdout,
            enable_broker_log: enable_broker_log,
            debug: debug,
            trace: trace,
            filea: RollingFileAppender::builder()
                .rotation(Rotation::DAILY) // rotate log files once every day
                .filename_prefix("platform") // log file names will be prefixed
                .filename_suffix("csv") // log file names will be suffixed with `.log`
                .max_log_files(3) // last 3 day stored
                .build(system_default_log_dir().unwrap())
                .unwrap(),
        }
    }
}

impl std::io::Write for MultiWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        //
        // Store in log file
        self.filea.write_all(buf).unwrap();

        //
        // Stdout logs ?
        if self.enable_stdout {
            print_log_line(buf, self.enable_broker_log, self.debug, self.trace);
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.filea.flush().unwrap();
        Ok(())
    }
}
