use anyhow::Result;
use d20::Roll;
use once_cell::sync::Lazy;
use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::model::application::command::Command;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::{Context, Mutex};
use serenity::utils::Color;

use crate::commands::choose::ChooseCommand;
use crate::commands::create_sheet::CreateSheetCommand;
use crate::commands::roll::RollCommand;
use crate::commands::set::SetCommand;
use crate::commands::skill::SkillCommand;
use crate::commands::status::StatusCommand;
use crate::database::SizedBotDatabase;

/// Represents a bot command.
#[serenity::async_trait]
pub trait BotCommand {
    /// Registers a command to Discord.
    fn register(&self, command: &mut CreateApplicationCommand);

    /// Gets a name of the command.
    fn name(&self) -> &str;

    /// Returns whether the command depends on a database.
    fn db_free(&self) -> bool;

    /// Executes the command.
    async fn execute(
        &self,
        ctx: &Context,
        interaction: &ApplicationCommandInteraction,
        data: &Mutex<SizedBotDatabase>,
    ) -> Result<Option<String>>;
}

/// The commands which can be invoked through the bot.
static REGISTERED_COMMANDS: Lazy<Vec<Box<dyn BotCommand + Sync + Send>>> = Lazy::new(|| {
    vec![
        Box::new(ChooseCommand),
        Box::new(CreateSheetCommand),
        Box::new(RollCommand),
        Box::new(SkillCommand),
        Box::new(SetCommand),
        Box::new(StatusCommand),
    ]
});

/// Controls all of commands.
pub struct BotCommandManager;

impl BotCommandManager {
    /// Registers all commands to Discord.
    pub async fn register_all(ctx: &Context, db_available: bool) -> Result<()> {
        Command::set_global_application_commands(ctx, |builder| {
            let commands = REGISTERED_COMMANDS
                .iter()
                .filter_map(|command| {
                    if db_available || !command.db_free() {
                        let mut builder = CreateApplicationCommand::default();
                        command.register(&mut builder);

                        println!("[BOT LOG] Registered /{}.", command.name());

                        Some(builder)
                    } else {
                        None
                    }
                })
                .collect();
            builder.set_application_commands(commands)
        })
        .await?;

        println!("[BOT LOG] Registered all commands.");

        Ok(())
    }

    /// Executes a command.
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

    /// Reports an error to the user.
    ///
    /// This method cannot be used to report an internal server error.
    async fn reply_error(
        ctx: &Context,
        interaction: &ApplicationCommandInteraction,
        error: String,
    ) -> Result<()> {
        interaction
            .send_embed(ctx, |embed| {
                embed.title("ERROR");
                embed.field("Message", error, false);
                embed.color(Color::RED);
                embed
            })
            .await?;

        Ok(())
    }
}

/// An extension for `ApplicationCommandInteraction`.
pub trait InteractionUtil {
    /// Gets a nickname of the user who invoked the command.
    fn get_nickname(&self) -> String;

    /// Gets a value of option as `String`.
    fn get_string_option(&self, name: String) -> Option<&str>;

    /// Gets a value of option as `i32`.
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

/// An extension for `ApplicationCommandInteraction` to send an embed content easily.
#[serenity::async_trait]
pub trait SendEmbed<F, 'l>
where
    F: FnOnce(&mut CreateEmbed) -> &mut CreateEmbed + Send + 'l,
{
    /// Sends an embed to the user.
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

/// Provides the way to convert `Roll` into `String`.
pub trait AsString {
    /// Converts `Roll` into `String`.
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

pub mod choose;
pub mod create_sheet;
pub mod roll;
pub mod set;
pub mod skill;
pub mod status;
