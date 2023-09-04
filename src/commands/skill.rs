use anyhow::Result;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::{Context, Mutex};

use crate::commands::{BotCommand, CommandStatus, InteractionUtil, SendEmbed};

/// A command that does a skill roll. It follows Call of Cthulhu 6th Edition.
pub struct SkillCommand;

/// A command that does a skill roll. It follows Call of Cthulhu 6th Edition.
pub struct Sk6Command;

/// A command that does a skill roll. It follows Call of Cthulhu 7th Edition.
pub struct Sk7Command;

/// A command that does a skill roll. It follows Delta Green.
pub struct SkDGCommand;

/// A command that does a skill roll. It follows the BRP 2023 rule book.
pub struct SkBRPCommand;

impl SkillCommand {
    /// Does a skill roll following the rule of Call of Cthulhu 6th Edition.
    async fn execute_6th(
        ctx: &Context,
        interaction: &ApplicationCommandInteraction,
    ) -> Result<CommandStatus> {
        let chance = interaction.get_int_option("chance".to_string()).unwrap();

        let comment = interaction
            .get_string_option("comment".to_string())
            .unwrap_or("a skill");

        let (result, roll) = match d20::roll_dice("1d100").unwrap().total {
            result if (result == 1 && result <= chance) => (
                ":star::crown::star: **Critical!!!**",
                format!("1 <= {}", chance),
            ),
            result if result <= 5 && (result <= chance) => {
                (":crown: **Critical!**", format!("{} <= {}", result, chance))
            }
            result if result == 100 && (result > chance) => (
                ":fire::skull::fire: **Fumble!!!**",
                format!("100 > {}", chance),
            ),
            result if result > 95 && (result > chance) => {
                (":skull: **Fumble!**", format!("{} > {}", result, chance))
            }
            result if result <= chance => (":o: **Success**", format!("{} <= {}", result, chance)),
            result => (":x: **Failed**", format!("{} > {}", result, chance)),
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
        let chance = interaction.get_int_option("chance".to_string()).unwrap();

        let comment = interaction
            .get_string_option("comment".to_string())
            .unwrap_or("a skill");

        let (result, roll) = match d20::roll_dice("1d100").unwrap().total {
            result if (result == 1 && result <= chance) => (
                ":star::crown::star: **Critical!!!**",
                format!("1 <= {}", chance),
            ),
            result if result <= chance / 5 => (
                ":crown: **Extreme Success!**",
                format!("{} <= {} / 5", result, chance),
            ),
            result if result <= chance / 2 => (
                ":o: **Hard Success!**",
                format!("{} <= {} / 2", result, chance),
            ),
            result if result == 100 || (result > 95 && chance < 50) => {
                (":skull: **Fumble!**", format!("{} >= {}", result, chance))
            }
            result if result <= chance => (":o: **Success**", format!("{} <= {}", result, chance)),
            result => (":x: **Failed**", format!("{} > {}", result, chance)),
        };

        interaction
            .send_embed(ctx, |embed| {
                embed.title(format!("{} uses {}", interaction.get_nickname(), comment));
                embed.field(result, roll, false)
            })
            .await?;

        Ok(CommandStatus::Ok)
    }

    async fn execute_dg(
        ctx: &Context,
        interaction: &ApplicationCommandInteraction,
    ) -> Result<CommandStatus> {
        let chance = interaction.get_int_option("chance".to_string()).unwrap();

        let comment = interaction
            .get_string_option("comment".to_string())
            .unwrap_or("a skill");

        let (result, roll) = match d20::roll_dice("1d100").unwrap().total {
            result if (result == 1 && result <= chance) => (
                ":star::crown::star: **Critical!!!**",
                format!("1 <= {}", chance),
            ),
            result if result <= 5 && (result <= chance) => {
                (":crown: **Critical!**", format!("{} <= {}", result, chance))
            }
            result if result / 10 == result % 10 && result <= chance => {
                (":crown: **Critical!**", format!("{} <= {}", result, chance))
            }
            result if result == 100 && (result > chance) => (
                ":fire::skull::fire: **Fumble!!!**",
                format!("100 > {}", chance),
            ),
            result if result > 95 && (result > chance) => {
                (":skull: **Fumble!**", format!("{} > {}", result, chance))
            }
            result if result <= chance => (":o: **Success**", format!("{} <= {}", result, chance)),
            result => (":x: **Failed**", format!("{} > {}", result, chance)),
        };

        interaction
            .send_embed(ctx, |embed| {
                embed.title(format!("{} uses {}", interaction.get_nickname(), comment));
                embed.field(result, roll, false)
            })
            .await?;

        Ok(CommandStatus::Ok)
    }

    async fn execute_brp(
        ctx: &Context,
        interaction: &ApplicationCommandInteraction,
    ) -> Result<CommandStatus> {
        let chance = interaction.get_int_option("chance".to_string()).unwrap();

        let comment = interaction
            .get_string_option("comment".to_string())
            .unwrap_or("a skill");

        let (result, roll) = match d20::roll_dice("1d100").unwrap().total {
            result if result <= (chance - 1) / 20 + 1 => (
                ":star::crown::star: **Critical!!!**",
                format!("{} <= {}", result, chance),
            ),
            result if result <= (chance - 1) / 5 + 1 => {
                (":crown: **Special!**", format!("{} <= {}", result, chance))
            }
            result if result >= i32::min(96 + (chance - 1) / 20, 100) && (result > chance) => {
                (":skull: **Fumble!**", format!("{} > {}", result, chance))
            }
            result if result <= chance => (":o: **Success**", format!("{} <= {}", result, chance)),
            result => (":x: **Failed**", format!("{} > {}", result, chance)),
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
            .description("Does a skill roll following the Call of Cthulhu 6th Edition. `/sk6` is the same. | 第6版のルールに基づいて技能ロールを行います. `/sk6`と同じ動きをします.")
            .create_option(|option| {
                option
                    .name("chance")
                    .kind(CommandOptionType::Integer)
                    .description("A skill chance | 技能値")
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
        Self::execute_6th(ctx, interaction).await
    }
}

#[naming]
#[db_required(false)]
#[serenity::async_trait]
impl BotCommand for Sk6Command {
    fn register(&self, command: &mut CreateApplicationCommand) {
        command
            .description("Does a skill roll following the Call of Cthulhu 6th Edition. | 第6版のルールに基づいて技能ロールを行います.")
            .create_option(|option| {
                option
                    .name("chance")
                    .kind(CommandOptionType::Integer)
                    .description("A skill chance | 技能値")
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
        SkillCommand::execute_6th(ctx, interaction).await
    }
}

#[naming]
#[db_required(false)]
#[serenity::async_trait]
impl BotCommand for Sk7Command {
    fn register(&self, command: &mut CreateApplicationCommand) {
        command
            .description("Does a skill roll following the Call of Cthulhu 7th Edition. | 第7版のルールに基づいて技能ロールを行います.")
            .create_option(|option| {
                option
                    .name("chance")
                    .kind(CommandOptionType::Integer)
                    .description("A skill chance | 技能値")
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

#[naming]
#[db_required(false)]
#[serenity::async_trait]
impl BotCommand for SkDGCommand {
    fn register(&self, command: &mut CreateApplicationCommand) {
        command
            .description("Does a skill roll following the Delta Green. | Delta Greenのルールに基づいて技能ロールを行います.")
            .create_option(|option| {
                option
                    .name("chance")
                    .kind(CommandOptionType::Integer)
                    .description("A skill chance | 技能値")
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
        SkillCommand::execute_dg(ctx, interaction).await
    }
}

#[naming]
#[db_required(false)]
#[serenity::async_trait]
impl BotCommand for SkBRPCommand {
    fn register(&self, command: &mut CreateApplicationCommand) {
        command
            .description("Does a skill roll following the BRP 2023. | BRP 2023のルールに基づいて技能ロールを行います.")
            .create_option(|option| {
                option
                    .name("chance")
                    .kind(CommandOptionType::Integer)
                    .description("A skill chance | 技能値")
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
        SkillCommand::execute_brp(ctx, interaction).await
    }
}
