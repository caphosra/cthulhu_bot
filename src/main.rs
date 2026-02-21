#[macro_use]
extern crate cmd_macro;

use anyhow::Result;
use log::info;
use once_cell::sync::Lazy;
use serenity::prelude::{GatewayIntents, Mutex};
use serenity::Client;

use crate::config::BotConfig;
use crate::database::{DummyDatabase, SizedBotDatabase};
use crate::handler::BotHandler;
use crate::logging::Logger;

#[cfg(feature = "db")]
use crate::database::PgDatabase;

/// A database that contains users information.
pub static DATABASE: Lazy<Mutex<SizedBotDatabase>> =
    Lazy::new(|| Mutex::new(Box::new(DummyDatabase {})));

/// Initializes a database.
#[cfg(feature = "db")]
async fn init_database(database_url: &Option<String>) -> Result<()> {
    if let Some(database_url) = database_url {
        let database = PgDatabase::init(database_url).await?;

        let mut db = DATABASE.lock().await;
        *db = Box::new(database);

        log!(LOG, "Initialized the database.");
    } else {
        log!(WARN, "No DATABASE_URL is provided.");
        log!(LOG, "Going to run without the database.");
    }

    Ok(())
}

/// Do nothing. If you want to use the database, please enable the `db` feature.
#[cfg(not(feature = "db"))]
async fn init_database(_database_url: &Option<String>) -> Result<()> {
    info!("The database features are disabled.");

    Ok(())
}

/// Initializes a bot and lets the bot start.
async fn start_bot() -> Result<()> {
    // Read the configurations.
    let config = BotConfig::get();

    // Connect to the database.
    init_database(&config.database_url).await?;

    // Build a client.
    let intents = GatewayIntents::empty();
    let mut client = Client::builder(&config.discord_token, intents)
        .event_handler(BotHandler)
        .await?;

    // Launch the client.
    client.start().await?;

    Ok(())
}

async fn start_process() -> Result<()> {
    // Initialize the file logging.
    Logger::init_file_logging().await?;

    info!(
        "----------------------\n  cthulhu bot v{}\n----------------------",
        env!("CARGO_PKG_VERSION")
    );

    Logger::enable_daily_reports();

    start_bot().await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    Logger::init();

    let result = start_process().await;
    Logger::log_err(&result).await;
}

pub mod commands;
pub mod config;
pub mod database;
pub mod handler;
pub mod logging;
