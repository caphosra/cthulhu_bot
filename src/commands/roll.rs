use anyhow::Result;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::{Context, Mutex};

use crate::commands::{AsString, BotCommand, InteractionUtil, SendEmbed};

/// A command to roll dices.
pub struct RollCommand;

#[serenity::async_trait]
#[naming]
#[db_required(false)]
impl BotCommand for RollCommand {
    fn register(&self, command: &mut CreateApplicationCommand) {
        command
            .description("Rolls designated dices. | 指定されたダイスを振ります.")
            .create_option(|option| {
                option
                    .name("dice")
                    .kind(CommandOptionType::String)
                    .description("An expression to be evaluated (ex. `3d4 + 1`) | 振りたいダイス (例: `3d4 + 1`)")
                    .required(true)
            })
            .create_option(|option| {
                option
                    .name("comment")
                    .kind(CommandOptionType::String)
                    .description("A comment | ダイスの説明")
            });
    }

    async fn execute(
        &self,
        ctx: &Context,
        interaction: &ApplicationCommandInteraction,
    ) -> Result<Option<String>> {
        let dice = interaction.get_string_option("dice".into()).unwrap();

        let comment = interaction
            .get_string_option("comment".into())
            .map(|comment| format!(" for {}", comment))
            .unwrap_or_default();

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
