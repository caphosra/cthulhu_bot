use anyhow::Result;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::{Context, Mutex};

use crate::commands::{BotCommand, CommandStatus, InteractionUtil, SendEmbed};

/// A command that does a skill roll. It follows Call of Cthulhu 5th Edition.
pub struct SkillCommand;

/// A command that does a skill roll. It follows Call of Cthulhu 5th Edition.
pub struct Sk5Command;

/// A command that does a skill roll. It follows Call of Cthulhu 7th Edition.
pub struct Sk7Command;

impl SkillCommand {
    /// Does a skill roll following the rule of Call of Cthulhu 5th Edition.
    async fn execute_5th(
        ctx: &Context,
        interaction: &ApplicationCommandInteraction,
    ) -> Result<CommandStatus> {
        let value = interaction.get_int_option("value".to_string()).unwrap();

        let comment = interaction
            .get_string_option("comment".to_string())
            .unwrap_or("a skill");

        let (result, roll) = match d20::roll_dice("1d100").unwrap().total {
            result if (result == 1 && result <= value) => (
                ":star::crown::star: **Critical!!!**",
                format!("1 <= {}", value),
            ),
            result if result <= 5 && (result <= value) => {
                (":crown: **Critical!**", format!("{} <= {}", result, value))
            }
            result if result == 100 && (result > value) => (
                ":fire::skull::fire: **Fumble!!!**",
                format!("100 > {}", value),
            ),
            result if result > 95 && (result > value) => {
                (":skull: **Fumble!**", format!("{} > {}", result, value))
            }
            result if result <= value => (":o: **Success**", format!("{} <= {}", result, value)),
            result => (":x: **Failed**", format!("{} > {}", result, value)),
        };

        interaction
            .send_embed(ctx, |embed| {
                embed.title(format!("{} uses {}", interaction.get_nickname(), comment));
                embed.field(result, roll, false)
            })
            .await?;

        Ok(CommandStatus::Ok)
    }

    /// Does a skill roll following the rule of Call of Cthulhu 7th Edition.
    async fn execute_7th(
        ctx: &Context,
        interaction: &ApplicationCommandInteraction,
    ) -> Result<CommandStatus> {
        let value = interaction.get_int_option("value".to_string()).unwrap();

        let comment = interaction
            .get_string_option("comment".to_string())
            .unwrap_or("a skill");

        let (result, roll) = match d20::roll_dice("1d100").unwrap().total {
            result if (result == 1 && result <= value) => (
                ":star::crown::star: **Critical!!!**",
                format!("1 <= {}", value),
            ),
            result if result <= value / 5 => (
                ":crown: **Extreme Success!**",
                format!("{} <= {} / 5", result, value),
            ),
            result if result <= value / 2 => (
                ":o: **Hard Success!**",
                format!("{} <= {} / 2", result, value),
            ),
            result if result == 100 || (result > 95 && value < 50) => {
                (":skull: **Fumble!**", format!("{} >= {}", result, value))
            }
            result if result <= value => (":o: **Success**", format!("{} <= {}", result, value)),
            result => (":x: **Failed**", format!("{} > {}", result, value)),
        };

        interaction
            .send_embed(ctx, |embed| {
                embed.title(format!("{} uses {}", interaction.get_nickname(), comment));
                embed.field(result, roll, false)
            })
            .await?;

        Ok(CommandStatus::Ok)
    }
}

#[naming]
#[db_required(false)]
#[serenity::async_trait]
impl BotCommand for SkillCommand {
    fn register(&self, command: &mut CreateApplicationCommand) {
        command
            .description("Does a skill roll. `/sk5` is the same. | 技能ロールを行います. `/sk5`と同じ動きをします.")
            .create_option(|option| {
                option
                    .name("value")
                    .kind(CommandOptionType::Integer)
                    .description("A skill value | 技能値")
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
    ) -> Result<CommandStatus> {
        Self::execute_5th(ctx, interaction).await
    }
}

#[naming]
#[db_required(false)]
#[serenity::async_trait]
impl BotCommand for Sk5Command {
    fn register(&self, command: &mut CreateApplicationCommand) {
        command
            .description("Does a skill roll under Call of Cthulhu 5th Edition. | 第5版に則り, 技能ロールを行います.")
            .create_option(|option| {
                option
                    .name("value")
                    .kind(CommandOptionType::Integer)
                    .description("A skill value | 技能値")
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
    ) -> Result<CommandStatus> {
        SkillCommand::execute_5th(ctx, interaction).await
    }
}

#[naming]
#[db_required(false)]
#[serenity::async_trait]
impl BotCommand for Sk7Command {
    fn register(&self, command: &mut CreateApplicationCommand) {
        command
            .description("Does a skill roll under Call of Cthulhu 7th Edition. | 第7版に則り, 技能ロールを行います.")
            .create_option(|option| {
                option
                    .name("value")
                    .kind(CommandOptionType::Integer)
                    .description("A skill value | 技能値")
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
    ) -> Result<CommandStatus> {
        SkillCommand::execute_7th(ctx, interaction).await
    }
}
