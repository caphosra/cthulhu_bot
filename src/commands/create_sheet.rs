use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::command_parser::*;
use crate::commands::*;

pub struct CreateSheetCommand;

#[serenity::async_trait]
impl BotCommand for CreateSheetCommand {
    fn is_valid(&self, info: &CommandInfo) -> bool {
        info.command == "create_sheet" || info.command == "cs"
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _info: &CommandInfo) -> Result<(), &'static str> {
        let mut out = String::new();

        let result = d20::roll_dice("3d6").unwrap();
        out += format!("Result:\n:dagger: STR **{}** ({})", result.total, roll_to_string(&result)).as_str();

        let result = d20::roll_dice("3d6").unwrap();
        out += format!("\n:umbrella: CON **{}** ({})", result.total, roll_to_string(&result)).as_str();

        let result = d20::roll_dice("3d6").unwrap();
        out += format!("\n:heart: POW **{}** ({})", result.total, roll_to_string(&result)).as_str();

        let result = d20::roll_dice("3d6").unwrap();
        out += format!("\n:dash: DEX **{}** ({})", result.total, roll_to_string(&result)).as_str();

        let result = d20::roll_dice("3d6").unwrap();
        out += format!("\n:star: APP **{}** ({})", result.total, roll_to_string(&result)).as_str();

        let result = d20::roll_dice("2d6+6").unwrap();
        out += format!("\n:elephant: SIZ **{}** ({})", result.total, roll_to_string(&result)).as_str();

        let result = d20::roll_dice("2d6+6").unwrap();
        out += format!("\n:bulb: INT **{}** ({})", result.total, roll_to_string(&result)).as_str();

        let result = d20::roll_dice("3d6+3").unwrap();
        out += format!("\n:books: EDU **{}** ({})", result.total, roll_to_string(&result)).as_str();

        let _ = msg.reply(ctx, &out)
            .await;

        Ok(())
    }
}
