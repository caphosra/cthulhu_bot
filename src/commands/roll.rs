use anyhow::Result;
use serenity::builder::{CreateCommand, CreateCommandOption, CreateEmbed};
use serenity::model::application::{CommandInteraction, CommandOptionType};
use serenity::prelude::{Context, Mutex};

use crate::commands::{AsString, BotCommand, CommandStatus, InteractionUtil, SendEmbed};

/// A command to roll dices.
pub struct RollCommand;

#[naming]
#[db_required(false)]
#[serenity::async_trait]
impl BotCommand for RollCommand {
    fn create(&self) -> CreateCommand {
        CreateCommand::new(self.name())
            .description("Rolls designated dices. | 指定されたダイスを振ります.")
            .add_option(
                CreateCommandOption::new(CommandOptionType::String, "dice", "An expression to be evaluated (ex. `3d4 + 1`) | 振りたいダイス (例: `3d4 + 1`)").required(true)
            )
            .add_option(
                CreateCommandOption::new(CommandOptionType::String, "comment", "A comment | ダイスの説明")
            )
    }

    async fn execute(
        &self,
        ctx: &Context,
        interaction: &CommandInteraction,
    ) -> Result<CommandStatus> {
        let dice = interaction.get_string_option("dice".into()).unwrap();

        let comment = interaction
            .get_string_option("comment".into())
            .map(|comment| format!(" for {}", comment))
            .unwrap_or_default();

        match d20::roll_dice(dice) {
            Ok(result) => {
                interaction
                    .send_embed(
                        ctx,
                        CreateEmbed::new()
                            .title(format!(
                                "{} rolls dice(s){}",
                                interaction.get_nickname(),
                                comment
                            ))
                            .field(
                                format!(":game_die: {}", result.total),
                                result.as_string(),
                                false,
                            ),
                    )
                    .await?;

                Ok(CommandStatus::Ok)
            }
            Err(err) => Ok(CommandStatus::Err(err.to_string())),
        }
    }
}
