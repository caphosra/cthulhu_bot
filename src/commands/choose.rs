use anyhow::Result;
use rand::Rng;
use serenity::builder::{CreateCommand, CreateCommandOption, CreateEmbed};
use serenity::model::application::{CommandInteraction, CommandOptionType};
use serenity::prelude::{Context, Mutex};

use crate::commands::{BotCommand, CommandStatus, InteractionUtil, SendEmbed};

/// A command that make a random choice.
pub struct ChooseCommand;

#[naming]
#[db_required(false)]
#[serenity::async_trait]
impl BotCommand for ChooseCommand {
    fn create(&self) -> CreateCommand {
        CreateCommand::new(self.name())
            .description("Makes a random choice.")
            .description_localized("ja", "与えられたものからランダムに選択をします.")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "choices",
                    "Comma-separated choices (ex. A,B,C)",
                )
                .name_localized("ja", "選択肢")
                .description_localized("ja", "カンマ区切りで与えてください (例: A,B,C)")
                .required(true),
            )
    }

    async fn execute(
        &self,
        ctx: &Context,
        interaction: &CommandInteraction,
    ) -> Result<CommandStatus> {
        let choices: Vec<&str> = interaction
            .get_string_option("choices".into())
            .unwrap()
            .split(",")
            .collect();

        let author = interaction.get_nickname();

        let selected = rand::thread_rng().gen_range(0..choices.len());

        interaction
            .send_embed(
                ctx,
                CreateEmbed::new()
                    .title(format!("{}'s choice", author))
                    .field(
                        format!("**{}**", choices[selected]),
                        format!("From {}", choices.join(",")),
                        false,
                    ),
            )
            .await?;

        Ok(CommandStatus::Ok)
    }
}
