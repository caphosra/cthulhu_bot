use std::collections::HashMap;
use std::env;
use std::thread;
use std::time;

use once_cell::sync::Lazy;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::Client;

pub mod command_parser;
pub mod commands;
pub mod user_data;

use crate::command_parser::*;
use crate::commands::*;
use crate::user_data::UserInfo;

/// The user data.
static USER_DATA: Lazy<Mutex<HashMap<u64, UserInfo>>> = Lazy::new(|| Mutex::new(HashMap::new()));

/// The path to the user data file.
static DATA_FILE_PATH: &str = "./user_info.coc";

/// The interval of saving the user data.
static SAVE_USER_DATA_INTERVAL: time::Duration = time::Duration::from_secs(60 * 5);

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
            run_command(&ctx, &msg, &info, &USER_DATA, false).await;
        }
    }
}

#[tokio::main]
async fn main() {
    // Load the environmental variables.
    let token = env::var("DISCORD_TOKEN").expect("Cannot gain a DISCORD_TOKEN.");
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

    // Load the user data.
    match UserInfo::load_file(DATA_FILE_PATH).await {
        Ok(data) => {
            let mut user_data = USER_DATA.lock().await;
            *user_data = data;
        }
        Err(err) => {
            println!("{}\nStart the bot without the data.", err);
        }
    }

    // Keep saving the user data asynchronously.
    tokio::spawn(async move {
        loop {
            thread::sleep(SAVE_USER_DATA_INTERVAL);

            let user_data = USER_DATA.lock().await;
            let _ = UserInfo::save_file(DATA_FILE_PATH, &*user_data).await;

            println!("Saved");
        }
    });

    // Launch the client.
    client.start().await.expect("Cannot start the bot.");
}
