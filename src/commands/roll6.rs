use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::command_parser::*;
use crate::commands::*;

pub struct Roll6Command;

#[serenity::async_trait]
impl BotCommand for Roll6Command {
    fn is_able_to_recurse(&self) -> bool {
        true
    }

    fn is_valid(&self, info: &CommandInfo) -> bool {
        info.command == "roll"
            || info.command == "roll6"
            || info.command == "r"
            || info.command == "r6"
    }

    async fn execute(
        &self,
        ctx: &Context,
        msg: &Message,
        info: &CommandInfo,
        _data: &Mutex<SizedBotDatabase>,
    ) -> Result<(), &'static str> {
        let limit: i32 = info
            .args
            .ok_or("You should provide at least one.")?
            .parse()
            .map_err(|_| "You should give a number.")?;
        let result = d20::roll_dice("1d100").unwrap().total;

        let result = match limit {
            limit => {
                if result == 1 && (result <= limit) {
                    format!(
                        "Result: :star::crown::star: **Critical!!!** (1 <= {})",
                        limit
                    )
                } else if result <= 5 && (result <= limit) {
                    format!("Result: :crown: **Critical!** ({} <= {})", result, limit)
                } else if result == 100 && (result > limit) {
                    format!(
                        "Result: :fire::skull::fire: **Fumble!!!** (100 > {})",
                        limit
                    )
                } else if result > 95 && (result > limit) {
                    format!("Result: :skull: **Fumble!** ({} > {})", result, limit)
                } else if result <= limit {
                    format!("Result: :o: **Success** ({} <= {})", result, limit)
                } else {
                    format!("Result: :x: **Failed** ({} > {})", result, limit)
                }
            }
        };
        let _ = msg.reply(ctx, result).await;

        Ok(())
    }
}
