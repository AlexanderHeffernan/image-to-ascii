use simplelog::*;
use std::fs::OpenOptions;
use log::{info, error};
use std::sync::OnceLock;

/// Ensures the logger is only initialized once for the entire application lifetime.
static LOGGER_INIT: OnceLock<()> = OnceLock::new();

/// A logger that tags each log entry with a unique request ID.
pub struct RequestLogger {
    request_id: i64,
}

impl RequestLogger {
    /// Initializes the logger if it hasn't been initialized yet.
    /// Sets up logging to a file with a custom time format and local time offset for NZT.
    fn init_logger() {
        LOGGER_INIT.get_or_init(|| {
            let time_format = format_description!("[day]/[month]/[year] [hour]:[minute]:[second]");
            let mut config = ConfigBuilder::new();
            config.set_time_format_custom(time_format);
            let _ = config.set_time_offset_to_local();
            let config = config.build();
        
            CombinedLogger::init(vec![
                WriteLogger::new(
                    LevelFilter::Info,
                    config,
                    OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open("backend.log")
                        .unwrap(),
                ),
            ]).unwrap();
        });
    }

    /// Create a new `RequestLogger` for a specific request ID.
    /// Logs the start of the request.
    pub fn new(request_id: i64) -> Self {
        Self::init_logger();
        info!("------------------- [Request {}] Start -------------------", request_id);
        Self { request_id }
    }

    /// Logs an informational message tagged with the request ID.
    pub fn info<S: AsRef<str>>(&self, msg: S) {
        info!("[{}] {}", self.request_id, msg.as_ref());
    }

    /// Logs an error message tagged with the request ID.
    pub fn error<S: AsRef<str>>(&self, msg: S) {
        error!("[{}] {}", self.request_id, msg.as_ref());
    }
}

impl Drop for RequestLogger {
    /// Logs the end of the request when the logger is dropped.
    /// This is called automatically when the `RequestLogger` goes out of scope.
    fn drop(&mut self) {
        info!("------------------- [Request {}] End ---------------------", self.request_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logger_can_be_created_and_logs_info() {
        let logger = RequestLogger::new(12345);
        logger.info("Test info message");
        logger.error("Test error message");
        // No assertions: just ensure no panic and log file is written.
    }
}