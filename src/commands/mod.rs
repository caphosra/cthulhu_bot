use std::collections::HashMap;

use d20::Roll;
use once_cell::sync::Lazy;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Color;

use crate::command_parser::*;
use crate::commands::create_sheet::*;
use crate::commands::custom_roll::*;
use crate::commands::define::DefineCommand;
use crate::commands::roll6::*;
use crate::commands::set::*;
use crate::commands::status::*;
use crate::commands::undefine::*;
use crate::commands::use_command::*;
use crate::user_data::UserInfo;

/// Represents a bot command.
#[serenity::async_trait]
pub trait BotCommand {
    fn is_able_to_recurse(&self) -> bool;
    fn is_valid(&self, info: &CommandInfo) -> bool;
    async fn execute(
        &self,
        ctx: &Context,
        msg: &Message,
        info: &CommandInfo,
        data: &Mutex<HashMap<u64, UserInfo>>,
    ) -> Result<(), &'static str>;
}

/// The commands which can be invoked through the bot.
static REGISTERED_COMMANDS: Lazy<Vec<Box<dyn BotCommand + Sync + Send>>> = Lazy::new(|| {
    vec![
        Box::new(CreateSheetCommand),
        Box::new(CustomRollCommand),
        Box::new(DefineCommand),
        Box::new(Roll6Command),
        Box::new(SetCommand),
        Box::new(StatusCommand),
        Box::new(UndefineCommand),
        Box::new(UseCommand),
    ]
});

async fn reply_error(ctx: &Context, msg: &Message, error: &str) {
    let _ = msg
        .channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.title("ERROR");
                e.field("Message", error, false);
                e.color(Color::RED);

                e
            });
            m.reference_message(msg);

            m
        })
        .await;
}

pub async fn run_command<'ctx>(
    ctx: &Context,
    msg: &Message,
    info: &CommandInfo<'ctx>,
    data: &Mutex<HashMap<u64, UserInfo>>,
    recursive: bool,
) {
    for command in REGISTERED_COMMANDS.iter() {
        if command.is_valid(&info) && (!recursive || command.is_able_to_recurse()) {
            let result = command.execute(ctx, msg, info, data).await;

            if let Err(message) = result {
                reply_error(ctx, msg, message).await;
            };
            return;
        }
    }
}

/// Convert `Roll` into `String`.
fn roll_to_string(roll: &Roll) -> String {
    let mut out = String::new();

    for i in 0..roll.values.len() {
        let ref val = roll.values[i];
        match val.0 {
            d20::DieRollTerm::Modifier(_) => out += format!("{}", &val.0).as_str(),
            d20::DieRollTerm::DieRoll { .. } => {
                out += format!("{}{:?}", &val.0, val.1).as_str();
            }
        }
    }

    out
}

pub mod create_sheet;
pub mod custom_roll;
pub mod define;
pub mod roll6;
pub mod set;
pub mod status;
pub mod undefine;
pub mod use_command;
