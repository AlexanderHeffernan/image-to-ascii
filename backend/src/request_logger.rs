use simplelog::*;
use std::fs::OpenOptions;
use log::{info, error};
use std::sync::OnceLock;
use chrono::{Local, TimeZone};
use chrono_tz::Pacific::Auckland;

static LOGGER_INIT: OnceLock<()> = OnceLock::new();

pub struct RequestLogger {
    request_id: i64,
}

impl RequestLogger {
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

    pub fn new(request_id: i64) -> Self {
        Self::init_logger();
        info!("------------------- [Request {}] Start -------------------", request_id);
        Self { request_id }
    }

    pub fn info<S: AsRef<str>>(&self, msg: S) {
        info!("[{}] {}", self.request_id, msg.as_ref());
    }

    pub fn error<S: AsRef<str>>(&self, msg: S) {
        error!("[{}] {}", self.request_id, msg.as_ref());
    }
}

impl Drop for RequestLogger {
    fn drop(&mut self) {
        info!("------------------- [Request {}] End ---------------------", self.request_id);
    }
}