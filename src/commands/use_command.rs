use std::collections::HashMap;

use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Color;

use crate::command_parser::*;
use crate::commands::*;
use crate::user_data::UserInfo;

pub struct UseCommand;

impl UseCommand {
    async fn send_commands_list(
        &self,
        ctx: &Context,
        msg: &Message,
        commands: &HashMap<String, String>,
    ) -> Result<(), &'static str> {
        let _ = msg
            .channel_id
            .send_message(&ctx, |m| {
                m.embed(|e| {
                    e.title(format!("{}'s commands", msg.author.name));

                    for (name, definition) in commands {
                        e.field(name, definition, true);
                    }
                    e.color(Color::PURPLE);

                    e
                });
                m.reference_message(msg);

                m
            })
            .await;

        Ok(())
    }
}

#[serenity::async_trait]
impl BotCommand for UseCommand {
    fn is_able_to_recurse(&self) -> bool {
        false
    }

    fn is_valid(&self, info: &CommandInfo) -> bool {
        info.command == "use" || info.command == "u"
    }

    async fn execute(
        &self,
        ctx: &Context,
        msg: &Message,
        info: &CommandInfo,
        data: &Mutex<HashMap<u64, UserInfo>>,
    ) -> Result<(), &'static str> {
        if let Some(args) = info.args {
            let solid_data = data.lock().await;
            if let Some(user_info) = solid_data.get(&msg.author.id.0) {
                if let Some(command) = user_info.commands.get(args) {
                    let info = parse_command(&command);

                    if let Some(info) = info {
                        Ok(run_command(ctx, msg, &info, data, true).await)
                    } else {
                        Err("Cannot execute the command.")
                    }
                } else {
                    Err("It is an unknown command.")
                }
            } else {
                Err("It is an unknown command.")
            }
        } else {
            let solid_data = data.lock().await;
            if let Some(user_info) = solid_data.get(&msg.author.id.0) {
                if user_info.commands.len() == 0 {
                    let _ = msg.reply_ping(&ctx, "There are no commands.").await;

                    Ok(())
                } else {
                    self.send_commands_list(ctx, msg, &user_info.commands).await
                }
            } else {
                let _ = msg.reply_ping(&ctx, "There are no commands.").await;

                Ok(())
            }
        }
    }
}
