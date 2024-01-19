use anyhow::Result;
use std::env;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufReader;

use once_cell::sync::Lazy;
use serde::Deserialize;
use serenity::prelude::Mutex;

/// Holds error information related to the process of loading the configurations.
/// This should be used to handle errors triggered before the logging system is up.
#[derive(Debug)]
pub struct BotConfigError {
    message: String,
}

impl Display for BotConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for BotConfigError {}

impl BotConfigError {
    pub fn new(message: &str) -> Self {
        BotConfigError {
            message: message.to_string(),
        }
    }
}

/// A set of configurations of this bot.
#[derive(Deserialize)]
pub struct BotConfig {
    pub discord_token: String,
    pub log_path: String,
    pub status_message: String,
    pub database_url: Option<String>,
}

/// Holds the configurations of this bot. You need to call `BotConfig::load_from_file` before using this.
pub static BOT_CONFIG: Lazy<Mutex<Option<BotConfig>>> = Lazy::new(|| Mutex::new(None));

impl BotConfig {
    /// Loads `BOT_CONFIG` from `./config.json`.
    pub async fn load_from_file() -> Result<()> {
        let executable_path = env::current_exe()?;
        let executable_dir = executable_path.parent().ok_or(BotConfigError::new(
            "Cannot retrieve the parent of this executable.",
        ))?;

        let config_file = executable_dir.join("config.json");
        if config_file.exists() {
            let config_file = File::open(config_file)?;
            let reader = BufReader::new(config_file);

            let config: BotConfig = serde_json::from_reader(reader)?;
            {
                let mut saved_config = BOT_CONFIG.lock().await;
                *saved_config = Some(config);
            }

            Ok(())
        } else {
            Err(BotConfigError::new(
                "No configuration file is provided. Create a new \"config.json\" in the directory where this bot is placed.",
            ))?
        }
    }
}
