use anyhow::Result;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::{Context, Mutex};

use crate::commands::{AsString, BotCommand, InteractionUtil, SendEmbed};
use crate::database::SizedBotDatabase;

/// A command that creates a character sheet.
pub struct CreateSheetCommand;

/// Adds a content to the embed.
macro_rules! add_content {
    ($embed:expr, $name:expr, $roll:expr) => {{
        let result = d20::roll_dice($roll).unwrap();
        $embed.field(
            format!("{} {}", $name, result.total),
            result.as_string(),
            true,
        );
    }};
}

#[serenity::async_trait]
impl BotCommand for CreateSheetCommand {
    fn register(&self, command: &mut CreateApplicationCommand) {
        command
            .name("cs")
            .description("Creates a character sheet. | キャラクターシートを作成します.");
    }

    fn name(&self) -> &str {
        "cs"
    }

    async fn execute(
        &self,
        ctx: &Context,
        interaction: &ApplicationCommandInteraction,
        _data: &Mutex<SizedBotDatabase>,
    ) -> Result<Option<String>> {
        let author = interaction.get_nickname();

        interaction
            .send_embed(ctx, |embed| {
                embed.title(format!("{}'s character", author));

                add_content!(embed, ":dagger: STR", "3d6");
                add_content!(embed, ":umbrella: CON", "3d6");
                add_content!(embed, ":heart: POW", "3d6");
                add_content!(embed, ":dash: DEX", "3d6");
                add_content!(embed, ":star: APP", "3d6");
                add_content!(embed, ":elephant: SIZ", "2d6+6");
                add_content!(embed, ":bulb: INT", "2d6+6");
                add_content!(embed, ":books: EDU", "3d6+3");

                embed
            })
            .await?;

        Ok(None)
    }
}
