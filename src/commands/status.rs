use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Color;

use crate::command_parser::*;
use crate::commands::*;

pub struct StatusCommand;

impl StatusCommand {
    async fn send_embed(
        &self,
        ctx: &Context,
        msg: &Message,
        hp: i16,
        san: i16,
        mp: i16,
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
    fn is_able_to_recurse(&self) -> bool {
        true
    }

    fn is_valid(&self, info: &CommandInfo) -> bool {
        info.command == "status" || info.command == "st"
    }

    async fn execute(
        &self,
        ctx: &Context,
        msg: &Message,
        _info: &CommandInfo,
        data: &Mutex<SizedBotDatabase>,
    ) -> Result<(), &'static str> {
        let mut data = data.lock().await;

        let info = data.get_value(msg.author.id.0).await;
        let _ = self.send_embed(ctx, msg, info.hp, info.san, info.mp).await;

        Ok(())
    }
}
