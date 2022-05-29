use std::env;
use std::thread;
use std::time;

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

/// The path to the user data file.
static DATA_FILE_PATH: &str = "./user_info.coc";

/// The interval of saving the user data.
static SAVE_USER_DATA_INTERVAL: time::Duration = time::Duration::from_secs(60 * 60 * 6);

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

#[tokio::main]
async fn main() {
    let database_url = env::var("DATABASE_URL").expect("Cannot gain a DATABASE_URL.");

    let database = PgDatabase::init(&database_url).await;

    let mut db = DATABASE.lock().await;
    *db = Box::new(database);

    // Load the environmental variables.
    let token = env::var("DISCORD_TOKEN").expect("Cannot gain a DISCORD_TOKEN.");

    // Build a client.
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Cannot setup a client.");

    // Launch the client.
    client.start().await.expect("Cannot start the bot.");
}
