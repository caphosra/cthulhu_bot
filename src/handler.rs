use serenity::model::application::interaction::{Interaction, InteractionType};
use serenity::model::prelude::{Activity, Ready};
use serenity::prelude::{Context, EventHandler};

use crate::commands::BotCommandManager;
use crate::config::BOT_CONFIG;
use crate::log;
use crate::logging::Logger;
use crate::DATABASE;

/// An event handler for the bot.
pub struct BotHandler;

#[serenity::async_trait]
impl EventHandler for BotHandler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        {
            let config = BOT_CONFIG.lock().await;
            assert!(config.is_some());

            let status_message = &config.as_ref().unwrap().status_message;
            ctx.set_activity(Activity::playing(status_message)).await;
        }

        let db = DATABASE.lock().await;
        let result = BotCommandManager::register_all(&ctx, db.is_available()).await;
        Logger::log_err(&result).await;

        log!(LOG, "{} is ready.", ready.user.name);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if interaction.kind() == InteractionType::ApplicationCommand {
            let interaction = interaction.application_command().unwrap();
            let result = BotCommandManager::run_command(&ctx, &interaction, &DATABASE).await;
            Logger::log_err(&result).await;
        }
    }
}
