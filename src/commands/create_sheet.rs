use anyhow::Result;
use once_cell::sync::Lazy;
use serenity::builder::{CreateCommand, CreateEmbed};
use serenity::model::application::CommandInteraction;
use serenity::prelude::{Context, Mutex};

use crate::commands::{AsString, BotCommand, CommandStatus, InteractionUtil, SendEmbed};

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

#[naming]
#[db_required(false)]
#[serenity::async_trait]
impl BotCommand for CSCommand {
    fn create(&self) -> CreateCommand {
        CreateCommand::new(self.name())
            .description("Creates a character sheet.")
            .description_localized("ja", "キャラクターシートを作成します.")
    }

    async fn execute(
        &self,
        ctx: &Context,
        interaction: &CommandInteraction,
    ) -> Result<CommandStatus> {
        let author = interaction.get_nickname();

        let embed = CreateEmbed::new().title(format!("{}'s character", author));
        let embed = STATUSES.iter().fold(embed, |embed, status| {
            let result = d20::roll_dice(status.roll).unwrap();
            embed.field(
                format!("{} {}", status.name, result.total),
                result.as_string(),
                true,
            )
        });

        interaction.send_embed(ctx, embed).await?;

        Ok(CommandStatus::Ok)
    }
}
