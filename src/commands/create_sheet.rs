use anyhow::Result;
use once_cell::sync::Lazy;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::{Context, Mutex};

use crate::commands::{AsString, BotCommand, InteractionUtil, SendEmbed};

/// A command that creates a character sheet.
pub struct CSCommand;

/// Represents a status which the character must have.
struct Status<'l> {
    pub name: &'l str,
    pub roll: &'l str,
}

/// A list of the statuses required.
static STATUSES: Lazy<Vec<Status>> = Lazy::new(|| {
    vec![
        Status {
            name: ":dagger: STR",
            roll: "3d6",
        },
        Status {
            name: ":umbrella: CON",
            roll: "3d6",
        },
        Status {
            name: ":heart: POW",
            roll: "3d6",
        },
        Status {
            name: ":dash: DEX",
            roll: "3d6",
        },
        Status {
            name: ":star: APP",
            roll: "3d6",
        },
        Status {
            name: ":elephant: SIZ",
            roll: "2d6+6",
        },
        Status {
            name: ":bulb: INT",
            roll: "2d6+6",
        },
        Status {
            name: ":books: EDU",
            roll: "3d6+3",
        },
    ]
});

#[serenity::async_trait]
#[naming]
#[db_required(false)]
impl BotCommand for CSCommand {
    fn register(&self, command: &mut CreateApplicationCommand) {
        command.description("Creates a character sheet. | キャラクターシートを作成します.");
    }

    async fn execute(
        &self,
        ctx: &Context,
        interaction: &ApplicationCommandInteraction,
    ) -> Result<Option<String>> {
        let author = interaction.get_nickname();

        interaction
            .send_embed(ctx, |embed| {
                embed.title(format!("{}'s character", author));

                for status in STATUSES.iter() {
                    let result = d20::roll_dice(status.roll).unwrap();
                    embed.field(
                        format!("{} {}", status.name, result.total),
                        result.as_string(),
                        true,
                    );
                }

                embed
            })
            .await?;

        Ok(None)
    }
}
