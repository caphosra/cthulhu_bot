use std::collections::HashMap;

use d20::Roll;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::command_parser::*;
use crate::user_data::UserInfo;

/// Represents a bot command.
#[serenity::async_trait]
pub trait BotCommand {
    fn is_valid(&self, info: &CommandInfo) -> bool;
    async fn execute(
        &self,
        ctx: &Context,
        msg: &Message,
        info: &CommandInfo,
        data: &Mutex<HashMap<u64, UserInfo>>,
    ) -> Result<(), &'static str>;
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
pub mod roll6;
