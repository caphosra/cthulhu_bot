use std::collections::HashMap;

use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::command_parser::*;
use crate::commands::*;
use crate::user_data::UserInfo;

pub struct UndefineCommand;

#[serenity::async_trait]
impl BotCommand for UndefineCommand {
    fn is_able_to_recurse(&self) -> bool {
        false
    }

    fn is_valid(&self, info: &CommandInfo) -> bool {
        info.command == "undefine" || info.command == "undef"
    }

    async fn execute(
        &self,
        ctx: &Context,
        msg: &Message,
        info: &CommandInfo,
        data: &Mutex<HashMap<u64, UserInfo>>,
    ) -> Result<(), &'static str> {
        let args = info.args.ok_or("`/undefine` calls for one arguments.")?;

        let mut solid_data = data.lock().await;
        if let Some(data) = solid_data.get_mut(&msg.author.id.0) {
            data.commands.remove_entry(args);

            let _ = msg
                .reply_ping(&ctx, "The command was deleted successfully.")
                .await;

            Ok(())
        } else {
            Err("You haven't declared the command.")
        }
    }
}
