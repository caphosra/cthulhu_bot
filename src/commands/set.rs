use anyhow::Result;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::{Context, Mutex};

use crate::commands::{BotCommand, InteractionUtil, SendEmbed};
use crate::database::SizedBotDatabase;

/// A command that changes the parameters.
pub struct SetCommand;

/// A macro that is used inside `update_param`.
macro_rules! update_param_internal {
    ($status:tt, $param:tt, $value:expr) => {{
        let before = $status.$param;
        $status.$param = $value;
        before
    }};
}

/// Updates a parameter.
macro_rules! update_param {
    ($status:tt, $param:expr, $value:expr) => {{
        match $param {
            "HP" => update_param_internal!($status, hp, $value),
            "SAN" => update_param_internal!($status, san, $value),
            "MP" => update_param_internal!($status, mp, $value),
            _ => panic!("The parameter isn't valid."),
        }
    }};
}

#[serenity::async_trait]
impl BotCommand for SetCommand {
    #[name("set")]
    fn register(&self, command: &mut CreateApplicationCommand) {
        command
            .description("Assigns a value to your parameter. | パラメータに値を代入します.")
            .create_option(|option| {
                option
                    .name("param")
                    .kind(CommandOptionType::String)
                    .add_string_choice("HP", "HP")
                    .add_string_choice("SAN", "SAN")
                    .add_string_choice("MP", "MP")
                    .description("A parameter name | 代入先")
                    .required(true)
            })
            .create_option(|option| {
                option
                    .name("value")
                    .kind(CommandOptionType::Integer)
                    .description("A value | 代入する値")
                    .required(true)
            });
    }

    #[db_required(true)]
    async fn execute(
        &self,
        ctx: &Context,
        interaction: &ApplicationCommandInteraction,
        data: &Mutex<SizedBotDatabase>,
    ) -> Result<Option<String>> {
        let parameter = interaction.get_string_option("param".into()).unwrap();
        let value = interaction.get_int_option("value".into()).unwrap();

        if value > 32767 || value < 0 {
            return Ok(Some(
                "You must provide the value between 0 and 32767. | 値は 0以上 32767以下 にしてください."
                    .to_string(),
            ));
        }

        let value = value as i16;
        let data = data.lock().await;
        let author = interaction.get_nickname();
        let user_id = interaction.user.id.0;
        let mut status = data.get_value(user_id).await;

        match parameter {
            "HP" | "SAN" | "MP" => {
                let before = update_param!(status, parameter, value);
                data.set_value(user_id, status).await?;

                interaction.send_embed(ctx, |embed| {
                    embed.title(format!("{}'s status", author));
                    embed.field(
                        parameter,
                        format!("{} **{}** → **{}**", Self::get_icon(parameter), before, value),
                        false
                    )
                }).await?;

                Ok(None)
            },
            _ => Ok(Some(
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
