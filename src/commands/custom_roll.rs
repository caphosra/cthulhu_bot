use std::collections::HashMap;

use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::command_parser::*;
use crate::commands::*;
use crate::user_data::UserInfo;

pub struct CustomRollCommand;

#[serenity::async_trait]
impl BotCommand for CustomRollCommand {
    fn is_valid(&self, info: &CommandInfo) -> bool {
        info.command == "custom_roll" || info.command == "cr"
    }

    async fn execute(
        &self,
        ctx: &Context,
        msg: &Message,
        info: &CommandInfo,
        _data: &Mutex<HashMap<u64, UserInfo>>,
    ) -> Result<(), &'static str> {
        let expr = info.args.ok_or("You should provide at least one.")?;
        let result =
            d20::roll_dice(expr).map_err(|_| "You should provide a correct expression.")?;

        let result = format!(
            "Result: :game_die: **{}** ({})",
            result.total,
            roll_to_string(&result)
        );

        let _ = msg.reply(ctx, result).await;

        Ok(())
    }
}
