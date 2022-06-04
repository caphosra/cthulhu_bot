use anyhow::Result;
use serenity::builder::CreateApplicationCommand;
use serenity::model::interactions::application_command::{
    ApplicationCommandInteraction, ApplicationCommandOptionType,
};
use serenity::prelude::{Context, Mutex};

use crate::commands::{AsString, BotCommand, InteractionUtil, SendEmbed};
use crate::database::SizedBotDatabase;

/// A command to roll dices.
pub struct RollCommand;

#[serenity::async_trait]
impl BotCommand for RollCommand {
    fn register(&self, command: &mut CreateApplicationCommand) {
        command
            .name("roll")
            .description("Rolls designated dices. | 指定されたダイスを振ります.")
            .create_option(|option| {
                option
                    .name("dice")
                    .kind(ApplicationCommandOptionType::String)
                    .description("An expression to evaluate (ex. `3d4 + 1`) | 振りたいダイス (例: `3d4 + 1`)")
                    .required(true)
            })
            .create_option(|option| {
                option
                    .name("comment")
                    .kind(ApplicationCommandOptionType::String)
                    .description("A comment | ダイスの説明")
            });
    }

    fn name(&self) -> &str {
        "roll"
    }

    async fn execute(
        &self,
        ctx: &Context,
        interaction: &ApplicationCommandInteraction,
        _data: &Mutex<SizedBotDatabase>,
    ) -> Result<Option<String>> {
        let dice = interaction.get_string_option("dice".to_string()).unwrap();

        let comment = interaction
            .get_string_option("comment".to_string())
            .map(|comment| format!(" for {}", comment))
            .unwrap_or("".to_string());

        match d20::roll_dice(dice) {
            Ok(result) => {
                interaction
                    .send_embed(ctx, |embed| {
                        embed.title(format!(
                            "{} rolls dice(s){}",
                            interaction.get_nickname(),
                            comment
                        ));
                        embed.field(
                            format!(":game_die: {}", result.total),
                            result.as_string(),
                            false,
                        )
                    })
                    .await?;

                Ok(None)
            }
            Err(err) => Ok(Some(err.to_string())),
        }
    }
}
