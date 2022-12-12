use anyhow::Result;
use rand::Rng;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::{Context, Mutex};

use crate::commands::{BotCommand, InteractionUtil, SendEmbed};
use crate::database::SizedBotDatabase;

/// A command that make a random choice.
pub struct ChooseCommand;

#[serenity::async_trait]
impl BotCommand for ChooseCommand {
    fn register(&self, command: &mut CreateApplicationCommand) {
        command
            .name("choose")
            .description("Make a random choice. | 与えられたものからランダムに選択をします.")
            .create_option(|option| {
                option
                    .name("choices")
                    .kind(CommandOptionType::String)
                    .description("A comma-separated choices (ex. A,B,C) | カンマで区切られた選択肢 (例: A,B,C)")
                    .required(true)
            });
    }

    fn name(&self) -> &str {
        "choose"
    }

    async fn execute(
        &self,
        ctx: &Context,
        interaction: &ApplicationCommandInteraction,
        _data: &Mutex<SizedBotDatabase>,
    ) -> Result<Option<String>> {
        let choices: Vec<&str> = interaction
            .get_string_option("choices".into())
            .unwrap()
            .split(",")
            .collect();

        let author = interaction.get_nickname();

        let selected = rand::thread_rng().gen_range(0..choices.len());

        interaction
            .send_embed(ctx, |embed| {
                embed.title(format!("{}'s choice", author));
                embed.field(
                    format!("**{}**", choices[selected]),
                    format!("From {}", choices.join(",")),
                    false,
                )
            })
            .await?;

        Ok(None)
    }
}
