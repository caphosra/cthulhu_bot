use serenity::model::prelude::*;
use serenity::prelude::*;
use d20::Roll;

use crate::command_parser::*;

/// Represents a bot command.
#[serenity::async_trait]
pub trait BotCommand {
    fn is_valid(&self, info: &CommandInfo) -> bool;
    async fn execute(&self, ctx: &Context, msg: &Message, info: &CommandInfo) -> Result<(), &'static str>;
}

/// Convert `Roll` into `String`.
fn roll_to_string(roll: &Roll) -> String {
    let mut out = String::new();

    for i in 0..roll.values.len() {
        let ref val = roll.values[i];
        match val.0 {
            d20::DieRollTerm::Modifier(_) => {
                out += format!("{}", &val.0).as_str()
            }
            d20::DieRollTerm::DieRoll { .. } => {
                out += format!("{}{:?}", &val.0, val.1).as_str();
            }
        }
    }

    out
}

pub mod create_sheet;
pub mod custom_roll;
pub mod roll6;
