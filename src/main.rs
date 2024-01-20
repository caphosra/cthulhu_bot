#[macro_use]
extern crate cmd_macro;

use anyhow::Result;
use once_cell::sync::Lazy;
use serenity::prelude::{GatewayIntents, Mutex};
use serenity::Client;

use crate::config::{BotConfig, BOT_CONFIG};
use crate::database::{DummyDatabase, PgDatabase, SizedBotDatabase};
use crate::handler::BotHandler;
use crate::logging::Logger;

/// A database that contains users information.
pub static DATABASE: Lazy<Mutex<SizedBotDatabase>> =
    Lazy::new(|| Mutex::new(Box::new(DummyDatabase {})));

/// Initializes a bot and lets the bot start.
async fn start_bot() -> Result<()> {
    // Read the configurations.
    let (token, database_url) = {
        let config = BOT_CONFIG.lock().await;
        if config.is_none() {
            log!(ERROR, "The config has not been initialized.");
        }

        let bot_config = config.as_ref().unwrap();
        (
            bot_config.discord_token.clone(),
            bot_config.database_url.clone(),
        )
    };

    // Connect to the database.
    if let Some(database_url) = database_url {
        let database = PgDatabase::init(&database_url).await?;

        let mut db = DATABASE.lock().await;
        *db = Box::new(database);

        log!(LOG, "Initialized the database.");
    } else {
        log!(WARN, "No DATABASE_URL is provided.");
        log!(LOG, "Going to run without the database.");
    }

    // Build a client.
    let intents = GatewayIntents::empty();
    let mut client = Client::builder(&token, intents)
        .event_handler(BotHandler)
        .await?;

    // Launch the client.
    client.start().await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    // Load the configurations.
    let result = BotConfig::load_from_file().await;
    Logger::log_err(&result).await;

    Logger::init().await;

    log!(
        LOG,
        "----------------------\n  cthulhu bot v{}\n----------------------",
        env!("CARGO_PKG_VERSION")
    );

    Logger::publish_daily_reports();

    let result = start_bot().await;
    Logger::log_err(&result).await;
}

pub mod commands;
pub mod config;
pub mod database;
pub mod handler;
pub mod logging;
