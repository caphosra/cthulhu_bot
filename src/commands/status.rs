use anyhow::Result;
use serenity::builder::{CreateCommand, CreateEmbed};
use serenity::model::application::CommandInteraction;
use serenity::prelude::{Context, Mutex};

use crate::commands::{BotCommand, CommandStatus, InteractionUtil, SendEmbed};
use crate::database::SizedBotDatabase;

/// A command that displays the status.
pub struct StatusCommand;

#[naming]
#[db_required(true)]
#[serenity::async_trait]
impl BotCommand for StatusCommand {
    fn create(&self) -> CreateCommand {
        CreateCommand::new(self.name())
            .description("Displays your status. | ステータスを表示します.")
    }

    async fn execute(
        &self,
        ctx: &Context,
        interaction: &CommandInteraction,
        data: &Mutex<SizedBotDatabase>,
    ) -> Result<CommandStatus> {
        let nickname = interaction.get_nickname();
        let user_id = interaction.user.id.get();
        let status = data.lock().await.get_value(user_id).await;

        interaction
            .send_embed(
                ctx,
                CreateEmbed::new()
                    .title(format!("{}'s status", nickname))
                    .field("HP", format!(":heart: **{}**", status.hp), true)
                    .field("SAN", format!(":shield: **{}**", status.san), true)
                    .field("MP", format!(":comet: **{}**", status.mp), true),
            )
            .await?;

        Ok(CommandStatus::Ok)
    }
}
