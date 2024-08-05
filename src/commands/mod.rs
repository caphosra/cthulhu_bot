use anyhow::Result;
use d20::Roll;
use once_cell::sync::Lazy;
use serenity::builder::{
    CreateCommand, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage,
};
use serenity::model::application::{Command, CommandInteraction};
use serenity::model::colour::Colour;
use serenity::prelude::{Context, Mutex};

use crate::commands::choose::ChooseCommand;
use crate::commands::create_sheet::CSCommand;
use crate::commands::roll::RollCommand;
use crate::commands::set::SetCommand;
use crate::commands::opposed::{Op6Command, Op7Command};
use crate::commands::skill::{Sk6Command, Sk7Command, SkBRPCommand, SkDGCommand, SkillCommand};
use crate::commands::status::StatusCommand;
use crate::database::SizedBotDatabase;
use crate::log;
use crate::logging::EVENT_COUNTERS;

/// Represents a handled result of the command.
/// Note that you cannot use this for internal errors.
pub enum CommandStatus {
    Ok,
    Err(String),
}

/// Represents a bot command.
#[serenity::async_trait]
pub trait BotCommand {
    /// Registers a command to Discord.
    fn create(&self) -> CreateCommand;

    /// Gets a name of the command.
    fn name(&self) -> &str;

    /// Returns whether the command depends on a database.
    fn use_db(&self) -> bool;

    /// Executes the command.
    async fn execute(
        &self,
        ctx: &Context,
        interaction: &CommandInteraction,
        data: &Mutex<SizedBotDatabase>,
    ) -> Result<CommandStatus>;
}

/// The commands which can be invoked through the bot.
static REGISTERED_COMMANDS: Lazy<Vec<Box<dyn BotCommand + Sync + Send>>> = Lazy::new(|| {
    vec![
        Box::new(ChooseCommand),
        Box::new(CSCommand),
        Box::new(RollCommand),
        Box::new(Op6Command),
        Box::new(Op7Command),
        Box::new(Sk6Command),
        Box::new(Sk7Command),
        Box::new(SkDGCommand),
        Box::new(SkBRPCommand),
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
        let commands = REGISTERED_COMMANDS
            .iter()
            .filter_map(|command| {
                if db_available || !command.use_db() {
                    tokio::spawn(async move {
                        log!(LOG, "Registering /{}.", command.name());
                    });

                    Some(command.create())
                } else {
                    None
                }
            })
            .collect();

        Command::set_global_commands(ctx, commands).await?;

        log!(LOG, "Registered all commands.");

        Ok(())
    }

    /// Executes a command.
    pub async fn run_command(
        ctx: &Context,
        interaction: &CommandInteraction,
        data: &Mutex<SizedBotDatabase>,
    ) -> Result<()> {
        let mut command_executed = false;
        for command in REGISTERED_COMMANDS.iter() {
            let name = &interaction.data.name;
            if command.name() == name {
                if command_executed {
                    log!(ERROR, "Some commands are duplicated.");
                    return Ok(());
                }
                command_executed = true;

                let result = command.execute(ctx, interaction, data).await?;

                let mut counter = EVENT_COUNTERS.lock().await;
                let command_counter = counter.get_mut(name);
                if let Some(counter) = command_counter {
                    *counter += 1;
                } else {
                    counter.insert(name.clone(), 1);
                }

                if let CommandStatus::Err(message) = result {
                    Self::reply_error(ctx, interaction, message).await?;
                };
            }
        }
        if !command_executed {
            log!(ERROR, "Tried to execute an unknown command.");
        }
        Ok(())
    }

    /// Reports an error to the user.
    ///
    /// This method cannot be used to report an internal server error.
    async fn reply_error(
        ctx: &Context,
        interaction: &CommandInteraction,
        error: String,
    ) -> Result<()> {
        interaction
            .send_embed(
                ctx,
                CreateEmbed::default()
                    .title("ERROR")
                    .field("Message", error, false)
                    .colour(Colour::RED),
            )
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

impl InteractionUtil for CommandInteraction {
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
            .map(|option| option.value.as_str().unwrap())
    }

    fn get_int_option(&self, name: String) -> Option<i32> {
        self.data
            .options
            .iter()
            .find(|option| option.name == name)
            .map(|option| option.value.as_i64().unwrap() as i32)
    }
}

/// An extension for `ApplicationCommandInteraction` to send an embed content easily.
#[serenity::async_trait]
pub trait SendEmbed<'l> {
    /// Sends an embed to the user.
    async fn send_embed(&'l self, ctx: &Context, embed: CreateEmbed) -> Result<()>;
}

#[serenity::async_trait]
impl<'l> SendEmbed<'l> for CommandInteraction {
    async fn send_embed(&'l self, ctx: &Context, embed: CreateEmbed) -> Result<()> {
        self.create_response(
            &ctx,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::default().add_embed(embed),
            ),
        )
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
pub mod opposed;
pub mod roll;
pub mod set;
pub mod skill;
pub mod status;
