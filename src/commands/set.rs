use std::collections::HashMap;

use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Color;

use crate::command_parser::*;
use crate::commands::*;
use crate::user_data::UserInfo;

pub struct SetCommand;

impl SetCommand {
    async fn send_embed(
        &self,
        ctx: &Context,
        msg: &Message,
        title: &str,
        emoji: &str,
        previous: u8,
        new: u8,
    ) -> Result<(), &'static str> {
        let _ = msg
            .channel_id
            .send_message(&ctx, |m| {
                m.embed(|e| {
                    e.title(format!("{}'s status", msg.author.name));
                    e.field(title, format!("{} **{} -> {}**", emoji, previous, new), true);
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
impl BotCommand for SetCommand {
    fn is_valid(&self, info: &CommandInfo) -> bool {
        info.command == "set" || info.command == "s"
    }

    async fn execute(
        &self,
        ctx: &Context,
        msg: &Message,
        info: &CommandInfo,
        data: &Mutex<HashMap<u64, UserInfo>>,
    ) -> Result<(), &'static str> {
        let mut solid_data = data.lock().await;

        if let Some(user_info) = solid_data.get_mut(&msg.author.id.0) {
            let args = info.args.ok_or("`/set` calls for two arguments.")?;
            let args: Vec<&str> = args.split(' ').collect();
            if args.len() == 2 {
                let parameter = args[0];
                let value: u8 = args[1]
                    .parse()
                    .map_err(|_| "You should give me an integer value.")?;

                match parameter {
                    "HP" | "hp" => {
                        let previous = user_info.hp;
                        (*user_info).hp = value;

                        self.send_embed(ctx, msg, "HP", ":heart:", previous, value)
                            .await?;

                        Ok(())
                    }
                    "SAN" | "san" => {
                        let previous = user_info.san;
                        (*user_info).san = value;

                        self.send_embed(ctx, msg, "SAN", ":shield:", previous, value)
                            .await?;

                        Ok(())
                    }
                    "MP" | "mp" => {
                        let previous = user_info.mp;
                        (*user_info).mp = value;

                        self.send_embed(ctx, msg, "MP", ":comet:", previous, value)
                            .await?;

                        Ok(())
                    }
                    _ => Err("The parameter you suggested doesn't exist."),
                }
            } else {
                Err("`/set` calls for two arguments.")
            }
        } else {
            let user_info = UserInfo::new(msg.author.id.0);
            solid_data.insert(msg.author.id.0, user_info);

            self.execute(ctx, msg, info, data).await
        }
    }
}
