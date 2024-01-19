use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::Write;

use anyhow::Result;
use chrono::prelude::Local;
use once_cell::sync::Lazy;
use serenity::prelude::Mutex;
use tokio::time::Duration;

use crate::config::BOT_CONFIG;

/// A kind of the log.
pub enum LogKind {
    LOG,
    WARN,
    ERROR,
}

impl ToString for LogKind {
    fn to_string(&self) -> String {
        match self {
            &LogKind::LOG => "LOG".to_string(),
            &LogKind::WARN => "WARN".to_string(),
            &LogKind::ERROR => "ERROR".to_string(),
        }
    }
}

/// A handle of the log file. You cannot access this before the file is opened.
pub static LOG_FILE: Lazy<Mutex<Box<Option<File>>>> = Lazy::new(|| Mutex::new(Box::new(None)));

#[macro_export]
macro_rules! log {
    ($kind:ident, $($text:expr),*) => {
        crate::logging::Logger::log(crate::logging::LogKind::$kind, format!($($text),*)).await
    };
}

/// A handler for the log.
pub struct Logger;

impl Logger {
    /// Opens the log file and assign its handle to `LOG_FILE`.
    pub async fn init() {
        let config = BOT_CONFIG.lock().await;
        if config.is_none() {
            log!(ERROR, "The config has not been initialized.");
        }

        let log_path = &config.as_ref().unwrap().log_path;
        let file = OpenOptions::new().append(true).create(true).open(log_path);
        match file {
            Ok(file) => {
                let mut log_file = LOG_FILE.lock().await;
                *log_file = Box::new(Some(file));
            }
            Err(err) => log!(ERROR, "{}", err),
        }
    }

    /// Emits logs to the file.
    ///
    /// If the file has not been opened yet or it is readonly, the program is going to panic.
    pub async fn log(kind: LogKind, text: String) {
        let date = Local::now().to_rfc3339();
        let text = text
            .lines()
            .map(|line| {
                let kind = format!("[{}]", kind.to_string());
                format!("{:35} {:7} {}\n", date, kind, line)
            })
            .collect::<Vec<_>>()
            .concat();

        let mut file = LOG_FILE.lock().await;
        match file.as_mut() {
            Some(file) => {
                if let Err(err) = file.write_all(text.as_bytes()) {
                    eprint!("{}", text);
                    panic!("Failed to write the log to the file. (Info: {})", err);
                }
            }
            None => {
                eprint!("{}", text);
                panic!("No log file is opened.");
            }
        }
    }

    /// Emits error logs.
    pub async fn log_err(result: &Result<()>) {
        if let Err(err) = result {
            let text = err.to_string()
                + "\n"
                + &err
                    .chain()
                    .skip(1)
                    .map(|line| format!("  because: {}\n", line))
                    .collect::<Vec<_>>()
                    .concat();
            Logger::log(LogKind::ERROR, text).await
        }
    }

    /// Publishes a report of the events everyday.
    pub fn publish_daily_reports() {
        tokio::spawn(async {
            log!(LOG, "The daily report system is now enabled.");

            loop {
                {
                    let mut counter = EVENT_COUNTERS.lock().await;
                    let mut text = "Daily report\n".to_string();
                    if counter.len() == 0 {
                        text += "  Nothing";
                    } else {
                        text += &counter
                            .iter()
                            .map(|(key, value)| format!("  {}: {}\n", key, value))
                            .collect::<Vec<_>>()
                            .concat();
                    }
                    log!(LOG, "{}", text);

                    *counter = HashMap::new();
                    log!(LOG, "Initialize the event counters.");
                }
                tokio::time::sleep(Duration::from_secs(60 * 60 * 24)).await;
            }
        });
    }
}

/// Counters of the events.
pub static EVENT_COUNTERS: Lazy<Mutex<HashMap<String, u32>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
