use anyhow::Result;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::{Context, Mutex};

use crate::commands::{BotCommand, CommandStatus, InteractionUtil, SendEmbed};
use crate::database::SizedBotDatabase;

/// A command that displays the status.
pub struct StatusCommand;

#[naming]
#[db_required(true)]
#[serenity::async_trait]
impl BotCommand for StatusCommand {
    fn register(&self, command: &mut CreateApplicationCommand) {
        command.description("Displays your status. | ステータスを表示します.");
    }

    async fn execute(
        &self,
        ctx: &Context,
        interaction: &ApplicationCommandInteraction,
        data: &Mutex<SizedBotDatabase>,
    ) -> Result<CommandStatus> {
        let nickname = interaction.get_nickname();
        let user_id = interaction.user.id.0;
        let status = data.lock().await.get_value(user_id).await;

        interaction
            .send_embed(ctx, |embed| {
                embed.title(format!("{}'s status", nickname));
                embed.field("HP", format!(":heart: **{}**", status.hp), true);
                embed.field("SAN", format!(":shield: **{}**", status.san), true);
                embed.field("MP", format!(":comet: **{}**", status.mp), true)
            })
            .await?;

        Ok(CommandStatus::Ok)
    }
}
