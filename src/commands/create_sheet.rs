use anyhow::Result;
use serenity::builder::{CreateCommand, CreateEmbed};
use serenity::model::application::CommandInteraction;
use serenity::prelude::Context;
use tyche::dice::roller::FastRand;
use tyche::expr::Describe;
use tyche::Expr;

use crate::commands::{BotCommand, CommandStatus, InteractionUtil, SendEmbed};

/// A command that creates a character sheet.
pub struct CSCommand;

/// Represents a status which the character must have.
struct Status<'l> {
    pub name: &'l str,
    pub roll: &'l str,
}

/// A list of the statuses required.
const STATUSES: [Status; 8] = [
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
];

#[naming]
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

        let mut roller = FastRand::default();

        let embed = CreateEmbed::new().title(format!("{}'s character", author));
        let embed = STATUSES.iter().fold(embed, |embed, status| {
            let expr: Expr = status.roll.parse().unwrap();
            let result = expr.eval(&mut roller).unwrap();
            embed.field(
                format!("{} {}", status.name, result.calc().unwrap()),
                result.describe(None),
                true,
            )
        });

        interaction.send_embed(ctx, embed).await?;

        Ok(CommandStatus::Ok)
    }
}
