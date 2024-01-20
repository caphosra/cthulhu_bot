use anyhow::Result;
use serenity::builder::{CreateCommand, CreateCommandOption, CreateEmbed};
use serenity::model::application::{CommandInteraction, CommandOptionType};
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
    async fn execute_6th(ctx: &Context, interaction: &CommandInteraction) -> Result<CommandStatus> {
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
            .send_embed(
                ctx,
                CreateEmbed::new()
                    .title(format!("{} uses {}", interaction.get_nickname(), comment))
                    .field(result, roll, false),
            )
            .await?;

        Ok(CommandStatus::Ok)
    }

    /// Does a skill roll following the rule of Call of Cthulhu 7th Edition.
    async fn execute_7th(ctx: &Context, interaction: &CommandInteraction) -> Result<CommandStatus> {
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
            .send_embed(
                ctx,
                CreateEmbed::new()
                    .title(format!("{} uses {}", interaction.get_nickname(), comment))
                    .field(result, roll, false),
            )
            .await?;

        Ok(CommandStatus::Ok)
    }

    async fn execute_dg(ctx: &Context, interaction: &CommandInteraction) -> Result<CommandStatus> {
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
            .send_embed(
                ctx,
                CreateEmbed::new()
                    .title(format!("{} uses {}", interaction.get_nickname(), comment))
                    .field(result, roll, false),
            )
            .await?;

        Ok(CommandStatus::Ok)
    }

    async fn execute_brp(ctx: &Context, interaction: &CommandInteraction) -> Result<CommandStatus> {
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
            .send_embed(
                ctx,
                CreateEmbed::new()
                    .title(format!("{} uses {}", interaction.get_nickname(), comment))
                    .field(result, roll, false),
            )
            .await?;

        Ok(CommandStatus::Ok)
    }
}

#[naming]
#[db_required(false)]
#[serenity::async_trait]
impl BotCommand for SkillCommand {
    fn create(&self) -> CreateCommand {
        CreateCommand::new(self.name())
            .description("Does a skill roll. `/sk6` (The CoC 6th Edition) is the same. | `/sk6`と同様に, 第6版のルールに基づいて技能ロールを行います.")
            .add_option(CreateCommandOption::new(CommandOptionType::Integer, "chance", "A skill chance | 技能値").required(true))
            .add_option(CreateCommandOption::new(CommandOptionType::String, "comment", "A comment | ダイスの説明"))
    }

    async fn execute(
        &self,
        ctx: &Context,
        interaction: &CommandInteraction,
    ) -> Result<CommandStatus> {
        Self::execute_6th(ctx, interaction).await
    }
}

#[naming]
#[db_required(false)]
#[serenity::async_trait]
impl BotCommand for Sk6Command {
    fn create(&self) -> CreateCommand {
        CreateCommand::new(self.name())
            .description("Does a skill roll following the Call of Cthulhu 6th Edition. | 第6版のルールに基づいて技能ロールを行います.")
            .add_option(CreateCommandOption::new(CommandOptionType::Integer, "chance", "A skill chance | 技能値").required(true))
            .add_option(CreateCommandOption::new(CommandOptionType::String, "comment", "A comment | ダイスの説明"))
    }

    async fn execute(
        &self,
        ctx: &Context,
        interaction: &CommandInteraction,
    ) -> Result<CommandStatus> {
        SkillCommand::execute_6th(ctx, interaction).await
    }
}

#[naming]
#[db_required(false)]
#[serenity::async_trait]
impl BotCommand for Sk7Command {
    fn create(&self) -> CreateCommand {
        CreateCommand::new(self.name())
            .description("Does a skill roll following the Call of Cthulhu 7th Edition. | 第7版のルールに基づいて技能ロールを行います.")
            .add_option(CreateCommandOption::new(CommandOptionType::Integer, "chance", "A skill chance | 技能値").required(true))
            .add_option(CreateCommandOption::new(CommandOptionType::String, "comment", "A comment | ダイスの説明"))
    }

    async fn execute(
        &self,
        ctx: &Context,
        interaction: &CommandInteraction,
    ) -> Result<CommandStatus> {
        SkillCommand::execute_7th(ctx, interaction).await
    }
}

#[naming]
#[db_required(false)]
#[serenity::async_trait]
impl BotCommand for SkDGCommand {
    fn create(&self) -> CreateCommand {
        CreateCommand::new(self.name())
            .description("Does a skill roll following the Delta Green. | Delta Greenのルールに基づいて技能ロールを行います.")
            .add_option(CreateCommandOption::new(CommandOptionType::Integer, "chance", "A skill chance | 技能値").required(true))
            .add_option(CreateCommandOption::new(CommandOptionType::String, "comment", "A comment | ダイスの説明"))
    }

    async fn execute(
        &self,
        ctx: &Context,
        interaction: &CommandInteraction,
    ) -> Result<CommandStatus> {
        SkillCommand::execute_dg(ctx, interaction).await
    }
}

#[naming]
#[db_required(false)]
#[serenity::async_trait]
impl BotCommand for SkBRPCommand {
    fn create(&self) -> CreateCommand {
        CreateCommand::new(self.name())
            .description("Does a skill roll following the BRP 2023. | BRP 2023のルールに基づいて技能ロールを行います.")
            .add_option(CreateCommandOption::new(CommandOptionType::Integer, "chance", "A skill chance | 技能値").required(true))
            .add_option(CreateCommandOption::new(CommandOptionType::String, "comment", "A comment | ダイスの説明"))
    }

    async fn execute(
        &self,
        ctx: &Context,
        interaction: &CommandInteraction,
    ) -> Result<CommandStatus> {
        SkillCommand::execute_brp(ctx, interaction).await
    }
}
