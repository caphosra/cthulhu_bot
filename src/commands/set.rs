use anyhow::Result;
use serenity::builder::{CreateCommand, CreateCommandOption, CreateEmbed};
use serenity::model::application::{CommandInteraction, CommandOptionType};
use serenity::prelude::{Context, Mutex};

use crate::commands::{BotCommand, CommandStatus, InteractionUtil, SendEmbed};
use crate::database::SizedBotDatabase;

/// A command that changes the parameters.
pub struct SetCommand;

#[naming]
#[db_required(true)]
#[serenity::async_trait]
impl BotCommand for SetCommand {
    fn create(&self) -> CreateCommand {
        CreateCommand::new(self.name())
            .description("Assigns a value to your parameter.")
            .description_localized("ja", "パラメータに値を代入します.")
            .add_option(
                CreateCommandOption::new(CommandOptionType::String, "param", "A parameter name")
                    .name_localized("ja", "パラメータ")
                    .description_localized("ja", "代入先")
                    .add_string_choice("HP", "HP")
                    .add_string_choice("SAN", "SAN")
                    .add_string_choice("MP", "MP")
                    .required(true),
            )
            .add_option(
                CreateCommandOption::new(CommandOptionType::Integer, "value", "A value")
                    .name_localized("ja", "値")
                    .description_localized("ja", "代入する値")
                    .required(true),
            )
    }

    async fn execute(
        &self,
        ctx: &Context,
        interaction: &CommandInteraction,
        data: &Mutex<SizedBotDatabase>,
    ) -> Result<CommandStatus> {
        let parameter = interaction.get_string_option("param".into()).unwrap();
        let value = interaction.get_int_option("value".into()).unwrap();

        if value > 32767 || value < 0 {
            return Ok(CommandStatus::Err(
                "You must provide a value between 0 and 32767. | 値は 0以上 32767以下 にしてください."
                    .to_string(),
            ));
        }

        let value = value as i16;
        let data = data.lock().await;
        let author = interaction.get_nickname();
        let user_id = interaction.user.id.get();
        let mut status = data.get_value(user_id).await;

        match parameter {
            "HP" | "SAN" | "MP" => {
                let before = status.update_value(parameter, value).unwrap();
                data.set_value(user_id, status).await?;

                interaction.send_embed(ctx, CreateEmbed::new()
                    .title(format!("{}'s status", author))
                    .field(
                        parameter,
                        format!("{} **{}** → **{}**", Self::get_icon(parameter), before, value),
                        false
                    )).await?;

                Ok(CommandStatus::Ok)
            },
            _ => Ok(CommandStatus::Err(
                format!(
                    "The parameter named \"{}\" doesn't exist. | \"{}\"という名前のパラメータはありません.",
                    parameter,
                    parameter
            )))
        }
    }
}

impl SetCommand {
    /// Gets an icon of the parameter.
    fn get_icon(parameter: &str) -> &str {
        match parameter {
            "HP" => ":heart:",
            "SAN" => ":shield:",
            "MP" => ":comet:",
            _ => "",
        }
    }
}
