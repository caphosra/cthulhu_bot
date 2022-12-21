use anyhow::Result;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::{Context, Mutex};

use crate::commands::{BotCommand, InteractionUtil, SendEmbed};

/// A command that does a skill roll.
pub struct SkillCommand;

#[naming]
#[db_required(false)]
#[serenity::async_trait]
impl BotCommand for SkillCommand {
    fn register(&self, command: &mut CreateApplicationCommand) {
        command
            .description("Attempts a skill roll. In other words, rolls 1d100. | 技能ロールを行います. 1d100を振って判定します.")
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
    ) -> Result<Option<String>> {
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

        Ok(None)
    }
}
