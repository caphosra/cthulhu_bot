#[macro_use]
extern crate cmd_macro;

use anyhow::Result;
use log::info;
use serenity::prelude::GatewayIntents;
use serenity::Client;

use crate::config::BotConfig;
use crate::handler::BotHandler;
use crate::logging::Logger;

/// Initializes a bot and lets the bot start.
async fn start_bot() -> Result<()> {
    // Read the configurations.
    let config = BotConfig::get();

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
pub mod handler;
pub mod logging;
