#[macro_use]
extern crate cmd_macro;

use std::env;

use anyhow::Result;
use once_cell::sync::Lazy;
use serenity::prelude::{GatewayIntents, Mutex};
use serenity::Client;

use crate::database::{DummyDatabase, PgDatabase, SizedBotDatabase};
use crate::error::DisplayErr;
use crate::handler::BotHandler;

/// A database that contains users information.
pub static DATABASE: Lazy<Mutex<SizedBotDatabase>> =
    Lazy::new(|| Mutex::new(Box::new(DummyDatabase {})));

/// A status message that shows up on the bot.
pub const STATUS_MESSAGE: &str = "Call of Cthulhu";

/// Initializes a bot and lets the bot start.
async fn start_bot() -> Result<()> {
    if let Ok(database_url) = env::var("DATABASE_URL") {
        let database = PgDatabase::init(&database_url).await?;

        let mut db = DATABASE.lock().await;
        *db = Box::new(database);

        println!("[BOT LOG] Initialized the database.")
    } else {
        println!("[BOT LOG] Is going to run without the database.")
    }

    // Load the environmental variables.
    let token = env::var("DISCORD_TOKEN")?;

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
    let result = start_bot().await;
    result.eprint_all();
}

pub mod commands;
pub mod database;
pub mod error;
pub mod handler;
