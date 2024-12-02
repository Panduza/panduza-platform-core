mod line_printer;

use line_printer::print_log_line;

///
///
///
pub struct MultiWriter {
    enable_stdout: bool,
    enable_broker_log: bool,
    filea: tracing_appender::rolling::RollingFileAppender,
}

impl MultiWriter {
    pub fn new(enable_stdout: bool, enable_broker_log: bool) -> Self {
        MultiWriter {
            enable_stdout: enable_stdout,
            enable_broker_log: enable_broker_log,
            filea: tracing_appender::rolling::never(".", "platform-log.csv"),
        }
    }
}

impl std::io::Write for MultiWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let buf_len = buf.len();

        self.filea.write_all(buf).unwrap();

        //
        // Stdout logs ?
        if self.enable_stdout {
            print_log_line(buf, self.enable_broker_log);
        }

        Ok(buf_len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.filea.flush().unwrap();
        Ok(())
    }
}
