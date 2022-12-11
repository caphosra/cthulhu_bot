use std::env;

use anyhow::Result;
use once_cell::sync::Lazy;
use serenity::model::prelude::{Activity, Interaction, InteractionType, Ready};
use serenity::prelude::{Context, EventHandler, GatewayIntents, Mutex};
use serenity::Client;

pub mod commands;
pub mod database;

use crate::commands::BotCommandManager;
use crate::database::{DummyDatabase, SizedBotDatabase};

/// A database that contains users information.
static DATABASE: Lazy<Mutex<SizedBotDatabase>> =
    Lazy::new(|| Mutex::new(Box::new(DummyDatabase {})));

/// Checks whether some errors occurred or not, and if did, reports those.
macro_rules! track_error {
    ($res: expr) => {
        if let Err(err) = $res {
            eprintln!("[BOT ERROR] {}", err);
            err.chain()
                .skip(1)
                .for_each(|cause| eprintln!("[BOT ERROR] because: {}", cause));
        }
    };
}

/// An event handler for the bot.
struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        ctx.set_activity(Activity::playing("Call of Cthulhu")).await;

        let result = BotCommandManager::register_all(&ctx).await;
        track_error!(result);

        println!("[BOT LOG] {} was connected.", ready.user.name);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction.kind() {
            InteractionType::ApplicationCommand => {
                let interaction = interaction.application_command().unwrap();
                let result = BotCommandManager::run_command(&ctx, &interaction, &DATABASE).await;
                track_error!(result);
            }
            _ => {}
        }
    }
}

/// Initializes a bot and lets the bot start.
async fn start_bot() -> Result<()> {
    // Load the environmental variables.
    let token = env::var("DISCORD_TOKEN")?;

    // Build a client.
    let intents = GatewayIntents::empty();
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await?;

    // Launch the client.
    client.start().await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let result = start_bot().await;
    track_error!(result);
}
