use std::env;

use anyhow::Result;
use once_cell::sync::Lazy;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::Client;

pub mod command_parser;
pub mod commands;
pub mod database;

use crate::command_parser::*;
use crate::commands::*;
use crate::database::{DummyDatabase, PgDatabase, SizedBotDatabase};

/// A database that contains users information.
static DATABASE: Lazy<Mutex<SizedBotDatabase>> =
    Lazy::new(|| Mutex::new(Box::new(DummyDatabase {})));

/// An event handler for the bot.
struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        ctx.set_activity(Activity::playing("Call of Cthulhu")).await;

        println!("{} is connected!", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        let info = parse_command(&msg.content);

        if let Some(info) = info {
            run_command(&ctx, &msg, &info, &DATABASE, false).await;
        }
    }
}

async fn start_bot() -> Result<()> {
    let database_url = env::var("DATABASE_URL")?;

    let database = PgDatabase::init(&database_url).await?;
    {
        let mut db = DATABASE.lock().await;
        *db = Box::new(database);
    }

    // Load the environmental variables.
    let token = env::var("DISCORD_TOKEN")?;

    // Build a client.
    let mut client = Client::builder(&token).event_handler(Handler).await?;

    // Launch the client.
    client.start().await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(err) = start_bot().await {
        eprintln!("[BOT ERROR] {}", err);
        err.chain()
            .skip(1)
            .for_each(|cause| eprintln!("[BOT ERROR] because: {}", cause));
        std::process::exit(1);
    }
}
