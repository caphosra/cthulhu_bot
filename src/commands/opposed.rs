use anyhow::Result;
use serenity::builder::{CreateCommand, CreateCommandOption, CreateEmbed};
use serenity::model::application::{CommandInteraction, CommandOptionType};
use serenity::prelude::{Context, Mutex};

use crate::commands::{BotCommand, CommandStatus, InteractionUtil, SendEmbed};

/// A command to do an opposed roll.
pub struct Op6Command;

#[naming]
#[db_required(false)]
#[serenity::async_trait]
impl BotCommand for Op6Command {
    fn create(&self) -> CreateCommand {
        CreateCommand::new(self.name())
            .description("Does an opposed roll following the Call of Cthulhu 6th Edition.")
            .description_localized("ja", "第6版のルールに基づいて対抗ロールをします.")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::Integer,
                    "status1",
                    "A status of player1 (ex. STR for a strength opposed roll.)",
                )
                .name_localized("ja", "参加者1")
                .description_localized("ja", "参加者1のステータス (例: STR対抗ならSTR.)")
                .required(true),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::Integer,
                    "status2",
                    "A status of player2 (ex. DEX for a dexterity opposed roll.)",
                )
                .name_localized("ja", "参加者2")
                .description_localized("ja", "参加者2のステータス (例: DEX対抗ならDEX.)")
                .required(true),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "name1",
                    "A name of player1",
                )
                .name_localized("ja", "名前1")
                .description_localized("ja", "参加者1の名前")
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "name2",
                    "A name of player2",
                )
                .name_localized("ja", "名前2")
                .description_localized("ja", "参加者2の名前")
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "comment",
                    "A comment for the roll",
                )
                .name_localized("ja", "コメント")
                .description_localized("ja", "ダイスの説明"),
            )
    }

    async fn execute(
        &self,
        ctx: &Context,
        interaction: &CommandInteraction,
    ) -> Result<CommandStatus> {
        let status1 = interaction.get_int_option("status1".into()).unwrap();
        let status2 = interaction.get_int_option("status2".into()).unwrap();

        if status1 < 0 || status1 > 20 || status2 < 0 || status2 > 20 {
            return Ok(CommandStatus::Err("A status must be 0-20.".to_string()));
        }

        let name1 = interaction.get_string_option("name1".into()).unwrap_or("player1");
        let name2 = interaction.get_string_option("name2".into()).unwrap_or("player2");

        let comment = interaction
            .get_string_option("comment".into())
            .unwrap_or("A competition");

        let chance = 50 + (status1 - status2) * 5;
        let chance = chance.clamp(0, 100);

        match d20::roll_dice("1d100") {
            Ok(result) => {
                let player1_won = result.total <= chance;
                let player1_result_text = format!("{} ({} <= {})?", status1, result.total, chance);
                let player2_result_text = format!("{} ({} > {})?", status2, result.total, chance);

                interaction
                    .send_embed(
                        ctx,
                        CreateEmbed::new()
                            .title(comment)
                            .field(
                                format!(":first_place: {}", if player1_won { name1 } else { name2 }),
                                if player1_won { &player1_result_text } else { &player2_result_text },
                                false,
                            )
                            .field(
                                format!(":second_place: {}", if player1_won { name2 } else { name1 }),
                                if player1_won { &player2_result_text } else { &player1_result_text },
                                false,
                            )
                    )
                    .await?;

                Ok(CommandStatus::Ok)
            }
            Err(err) => Ok(CommandStatus::Err(err.to_string())),
        }
    }
}
