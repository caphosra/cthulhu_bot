use std::collections::HashMap;

use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::command_parser::*;
use crate::commands::*;
use crate::user_data::UserInfo;

pub struct DefineCommand;

#[serenity::async_trait]
impl BotCommand for DefineCommand {
    fn is_able_to_recurse(&self) -> bool {
        false
    }

    fn is_valid(&self, info: &CommandInfo) -> bool {
        info.command == "define" || info.command == "def"
    }

    async fn execute(
        &self,
        ctx: &Context,
        msg: &Message,
        info: &CommandInfo,
        data: &Mutex<HashMap<u64, UserInfo>>,
    ) -> Result<(), &'static str> {
        let args = info.args.ok_or("`/define` calls for two arguments.")?;
        let args: Vec<&str> = args.splitn(2, ' ').collect();

        if args.len() != 2 {
            return Err("`/define` calls for two arguments.");
        }

        let command_name = args[0];
        let command = args[1];

        if command_name.as_bytes().contains(&b'\x03') || command.as_bytes().contains(&b'\x03') {
            return Err("The command contains `\\x03`, an invalid character.");
        }

        if command_name.len() > 20 || command.len() > 20 {
            return Err("The command is too long.");
        }

        let mut solid_data = data.lock().await;
        if let Some(data) = solid_data.get_mut(&msg.author.id.0) {
            if data.commands.len() > 5 {
                return Err(
                    "You can define the command up to five. You may use `/undef` to delete one.",
                );
            }

            data.commands
                .insert(command_name.to_string(), command.to_string());
            let _ = msg
                .reply_ping(&ctx, format!("You defined `{}`.", command_name))
                .await;

            Ok(())
        } else {
            let user_info = UserInfo::new(msg.author.id.0);
            solid_data.insert(msg.author.id.0, user_info);
            drop(solid_data);

            self.execute(ctx, msg, info, data).await
        }
    }
}
