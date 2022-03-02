use std::collections::HashMap;

use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Color;

use crate::command_parser::*;
use crate::commands::*;
use crate::user_data::UserInfo;

pub struct StatusCommand;

impl StatusCommand {
    async fn send_embed(
        &self,
        ctx: &Context,
        msg: &Message,
        hp: u8,
        san: u8,
        mp: u8,
    ) -> Result<(), &'static str> {
        let _ = msg
            .channel_id
            .send_message(&ctx, |m| {
                m.embed(|e| {
                    e.title(format!("{}'s status", msg.author.name));
                    e.field("HP", format!(":heart: **{}**", hp), true);
                    e.field("SAN", format!(":shield: **{}**", san), true);
                    e.field("MP", format!(":comet: **{}**", mp), true);
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
impl BotCommand for StatusCommand {
    fn is_valid(&self, info: &CommandInfo) -> bool {
        info.command == "status" || info.command == "st"
    }

    async fn execute(
        &self,
        ctx: &Context,
        msg: &Message,
        _info: &CommandInfo,
        data: &Mutex<HashMap<u64, UserInfo>>,
    ) -> Result<(), &'static str> {
        let mut data = data.lock().await;

        if let Some(info) = data.get(&msg.author.id.0) {
            let _ = self.send_embed(ctx, msg, info.hp, info.san, info.mp).await;
        } else {
            let info = UserInfo::new(msg.author.id.0);
            data.insert(msg.author.id.0, info);

            let _ = self.send_embed(ctx, msg, 0, 0, 0).await;
        }

        Ok(())
    }
}
