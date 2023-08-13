use std::env;
use std::fs::File;
use std::io::Write;

use anyhow::Result;
use chrono::prelude::Local;
use once_cell::sync::Lazy;
use serenity::prelude::Mutex;

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
        if let Ok(log_path) = env::var("LOG_PATH") {
            let file = File::open(log_path);
            match file {
                Ok(file) => {
                    let mut log_file = LOG_FILE.blocking_lock();
                    *log_file = Box::new(Some(file));
                }
                Err(err) => log!(ERROR, "{}", err),
            }
        } else {
            log!(ERROR, "Failed to get LOG_PATH.");
        }
    }

    /// Emits logs to the file.
    ///
    /// If the file has not been opened yet or it is readonly, the program is going to panic.
    pub async fn log(kind: LogKind, text: String) {
        let date = Local::now().to_rfc3339();
        let text = text
            .lines()
            .map(|line| format!("{} [{}] {}\n", date, kind.to_string(), line))
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
                    .map(|line| format!(" because: {}\n", line))
                    .collect::<Vec<_>>()
                    .concat();
            Logger::log(LogKind::ERROR, text).await
        }
    }
}
