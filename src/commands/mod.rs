use anyhow::Result;
use d20::Roll;
use once_cell::sync::Lazy;
use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::model::interactions::application_command::{
    ApplicationCommand, ApplicationCommandInteraction,
};
use serenity::prelude::{Context, Mutex};
use serenity::utils::Color;

use crate::commands::create_sheet::CreateSheetCommand;
use crate::commands::roll::RollCommand;
use crate::commands::set::SetCommand;
use crate::commands::skill::SkillCommand;
use crate::commands::status::StatusCommand;
use crate::database::SizedBotDatabase;

/// Represents a bot command.
#[serenity::async_trait]
pub trait BotCommand {
    fn register(&self, command: &mut CreateApplicationCommand);
    fn name(&self) -> &str;
    async fn execute(
        &self,
        ctx: &Context,
        interaction: &ApplicationCommandInteraction,
        data: &Mutex<SizedBotDatabase>,
    ) -> Result<Option<String>>;
}

pub struct BotCommandManager;

/// The commands which can be invoked through the bot.
static REGISTERED_COMMANDS: Lazy<Vec<Box<dyn BotCommand + Sync + Send>>> = Lazy::new(|| {
    vec![
        Box::new(CreateSheetCommand),
        Box::new(RollCommand),
        Box::new(SetCommand),
        Box::new(StatusCommand),
        Box::new(SkillCommand),
    ]
});

impl BotCommandManager {
    pub async fn register_all(ctx: &Context) -> Result<()> {
        ApplicationCommand::set_global_application_commands(ctx, |builder| {
            let commands = REGISTERED_COMMANDS
                .iter()
                .map(|command| {
                    let mut builder = CreateApplicationCommand::default();
                    command.register(&mut builder);

                    println!("[BOT LOG] Registered /{}.", command.name());

                    builder
                })
                .collect();
            builder.set_application_commands(commands)
        })
        .await?;

        println!("[BOT LOG] Registered all commands.");

        Ok(())
    }

    pub async fn run_command(
        ctx: &Context,
        interaction: &ApplicationCommandInteraction,
        data: &Mutex<SizedBotDatabase>,
    ) -> Result<()> {
        for command in REGISTERED_COMMANDS.iter() {
            if command.name() == interaction.data.name {
                let result = command.execute(ctx, interaction, data).await?;

                if let Some(message) = result {
                    Self::reply_error(ctx, interaction, message).await?;
                };
            }
        }
        Ok(())
    }

    pub async fn reply_error(
        ctx: &Context,
        interaction: &ApplicationCommandInteraction,
        error: String,
    ) -> Result<()> {
        interaction
            .create_interaction_response(&ctx, |builder| {
                builder.interaction_response_data(|builder| {
                    builder.embed(|e| {
                        e.title("ERROR");
                        e.field("Message", error, false);
                        e.color(Color::RED);
                        e
                    });
                    builder
                });
                builder
            })
            .await?;

        Ok(())
    }
}

pub trait InteractionUtil {
    fn get_nickname(&self) -> String;
    fn get_string_option(&self, name: String) -> Option<&str>;
    fn get_int_option(&self, name: String) -> Option<i32>;
}

impl InteractionUtil for ApplicationCommandInteraction {
    fn get_nickname(&self) -> String {
        match &self.member {
            Some(member) => member.display_name().to_string(),
            None => self.user.name.clone(),
        }
    }

    fn get_string_option(&self, name: String) -> Option<&str> {
        self.data
            .options
            .iter()
            .find(|option| option.name == name)
            .map(|option| option.value.as_ref().unwrap().as_str().unwrap())
    }

    fn get_int_option(&self, name: String) -> Option<i32> {
        self.data
            .options
            .iter()
            .find(|option| option.name == name)
            .map(|option| option.value.as_ref().unwrap().as_i64().unwrap() as i32)
    }
}

#[serenity::async_trait]
pub trait SendEmbed<F, 'l>
where
    F: FnOnce(&mut CreateEmbed) -> &mut CreateEmbed + Send + 'l,
{
    async fn send_embed(&'l self, ctx: &Context, f: F) -> Result<()>;
}

#[serenity::async_trait]
impl<F, 'l> SendEmbed<F, 'l> for ApplicationCommandInteraction
where
    F: (FnOnce(&mut CreateEmbed) -> &mut CreateEmbed) + Send + 'l,
{
    async fn send_embed(&'l self, ctx: &Context, f: F) -> Result<()> {
        self.create_interaction_response(&ctx, |res| {
            res.interaction_response_data(|res| res.embed(f))
        })
        .await?;
        Ok(())
    }
}

/// Convert `Roll` into `String`.
pub trait AsString {
    fn as_string(&self) -> String;
}

impl AsString for Roll {
    fn as_string(&self) -> String {
        let mut out = String::new();

        for i in 0..self.values.len() {
            let ref val = self.values[i];
            match val.0 {
                d20::DieRollTerm::Modifier(_) => out += format!("{}", &val.0).as_str(),
                d20::DieRollTerm::DieRoll { .. } => {
                    out += format!("{}{:?}", &val.0, val.1).as_str();
                }
            }
        }
        out
    }
}

pub mod create_sheet;
pub mod roll;
pub mod set;
pub mod skill;
pub mod status;
