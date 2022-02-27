use std::env;

use once_cell::sync::Lazy;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::Client;

pub mod commands;
pub mod command_parser;

use crate::commands::*;
use crate::commands::create_sheet::*;
use crate::commands::custom_roll::*;
use crate::commands::roll6::*;
use crate::command_parser::*;

/// The commands which can be invoked through the bot.
static REGISTERED_COMMANDS: Lazy<Vec<Box<dyn BotCommand + Sync + Send>>> = Lazy::new(|| {
    vec![
        Box::new(CreateSheetCommand),
        Box::new(CustomRollCommand),
        Box::new(Roll6Command)
    ]
});

/// An event handler for the bot.
struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        let info = parse_command(&msg.content);

        if let Some(info) = info {
            for command in REGISTERED_COMMANDS.iter() {
                if command.is_valid(&info) {
                    let result = command.execute(&ctx, &msg, &info)
                        .await;

                    if let Err(message) = result {
                        let _ = msg.reply(&ctx, message)
                            .await;
                    };
                    return;
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Load the environmental variables.
    let token = env::var("DISCORD_TOKEN")
        .expect("Cannot gain a DISCORD_TOKEN.");
    let application_id = env::var("APPLICATION_ID")
        .expect("Cannot gain an APPLICATION_ID.")
        .parse()
        .expect("Invalid APPLICATION_ID.");

    // Build a client.
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .application_id(application_id)
        .await
         .expect("Cannot setup a client.");

    // Launch the client.
    client.start()
        .await
        .expect("Cannot start the bot.");
}
