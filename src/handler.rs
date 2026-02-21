use log::info;
use serenity::gateway::ActivityData;
use serenity::model::application::{Interaction, InteractionType};
use serenity::model::prelude::Ready;
use serenity::prelude::{Context, EventHandler};

use crate::commands::BotCommandManager;
use crate::config::BotConfig;
use crate::logging::Logger;
use crate::DATABASE;

/// An event handler for the bot.
pub struct BotHandler;

#[serenity::async_trait]
impl EventHandler for BotHandler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        let config = BotConfig::get();

        ctx.set_activity(Some(ActivityData::playing(&config.status_message)));

        let db = DATABASE.lock().await;
        let result = BotCommandManager::register_all(&ctx, db.is_available()).await;
        Logger::log_err(&result).await;

        info!("{} is ready.", ready.user.name);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if interaction.kind() == InteractionType::Command {
            let interaction = interaction.command().unwrap();
            let result = BotCommandManager::run_command(&ctx, &interaction, &DATABASE).await;
            Logger::log_err(&result).await;
        }
    }
}
