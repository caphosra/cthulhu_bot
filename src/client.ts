import * as Discord from "discord.js";
import DiceExpression from "dice-expression-evaluator";

//
// Roll a d100 dice.
//
function roll_d100(msg: Discord.Message, args: string | undefined, comment: string | undefined) {
    let number = -1;

    if (args) {
        const number_regex = /\d+/;
        const matched_number = args.match(number_regex);
        if (matched_number) {
            number = parseInt(matched_number[0]);
        }
    }

    const expr = new DiceExpression("d100");
    let result = expr.roll().roll;

    let result_message = "";
    if (result == 1 && (number == -1 || result <= number)) {
        result_message = `:star::crown::star: **Critical!!!** (${result})`;
    }
    else if (result == 100 && (number == -1 || result > number)) {
        result_message = `:fire::skull::fire: **Fumble!!!** (${result})`;
    }
    else if (result <= 5 && (number == -1 || result <= number)) {
        result_message = `:crown: **Critical!** (${result})`;
    }
    else if (result > 95  && (number == -1 || result > number)) {
        result_message = `:skull: **Fumble!** (${result})`;
    }
    else if (number == -1) {
        result_message = `:question: **Can't judge** (${result})`;
    }
    else if (result <= number) {
        result_message = `:o: **Success** (${result} <= ${number})`;
    }
    else {
        result_message = `:x: **Failed** (${result} > ${number})`;
    }

    if (comment) {
        msg.reply(`\"${comment}\" Result: ${result_message}`);
    }
    else {
        msg.reply(`Result: ${result_message}`);
    }
}

//
// Roll dices which are not d100.
//
function roll_dice(msg: Discord.Message, args: string | undefined, comment: string | undefined) {
    if (!args) {
        msg.reply("You have to give me a correct expression.");
        return;
    }

    try {
        const expr = new DiceExpression(args);
        let result = expr.roll();

        let result_message = `:game_die: **${result.roll}** (${args} : ${[].concat(...result.diceRaw).join(", ")})`;

        if (comment) {
            msg.reply(`\"${comment}\" Result: ${result_message}`);
        }
        else {
            msg.reply(`Result: ${result_message}`);
        }
    }
    catch {
        msg.reply("You have to give me a correct expression.");
    }
}

//
// Create a character sheet.
//
function create_sheet(msg: Discord.Message, args: string | undefined, comment: string | undefined) {
    const expr_3d6 = new DiceExpression("3d6");
    const expr_2d6_6 = new DiceExpression("2d6+6");
    const expr_3d6_3 = new DiceExpression("3d6+3");

    let result = expr_3d6.roll();
    let message = `Result:\n:dagger: STR **${result.roll}** (3d6 : ${[].concat(...result.diceRaw).join(", ")})`;
    result = expr_3d6.roll();
    message += `\n:umbrella: CON **${result.roll}** (3d6 : ${[].concat(...result.diceRaw).join(", ")})`;
    result = expr_3d6.roll();
    message += `\n:heart: POW **${result.roll}** (3d6 : ${[].concat(...result.diceRaw).join(", ")})`;
    result = expr_3d6.roll();
    message += `\n:dash: DEX **${result.roll}** (3d6 : ${[].concat(...result.diceRaw).join(", ")})`;
    result = expr_3d6.roll();
    message += `\n:star: APP **${result.roll}** (3d6 : ${[].concat(...result.diceRaw).join(", ")})`;
    result = expr_2d6_6.roll();
    message += `\n:elephant: SIZ **${result.roll}** (2d6 + 6 : ${[].concat(...result.diceRaw).join(", ")})`;
    result = expr_2d6_6.roll();
    message += `\n:bulb: INT **${result.roll}** (2d6 + 6 : ${[].concat(...result.diceRaw).join(", ")})`;
    result = expr_3d6_3.roll();
    message += `\n:books: EDU **${result.roll}** (3d6 + 3 : ${[].concat(...result.diceRaw).join(", ")})`;

    if (comment) {
        msg.reply(`\"${comment}\" ${message}`);
    }
    else {
        msg.reply(message);
    }
}

//
// Process the args.
//
function calculate_args(msg: Discord.Message) {
    const arg_regex = /^\/([^#\s]+)(\s+([^#]+))?(\s*#(.+))?/;
    const matched_arg = msg.content.match(arg_regex);

    if (!matched_arg) {
        return;
    }

    const param_name = matched_arg[1];
    const args = matched_arg[3];
    const comment = matched_arg[5];

    if (param_name == "r" || param_name == "roll") {
        roll_d100(msg, args, comment);
    }

    if (param_name == "cr" || param_name == "custom_roll") {
        roll_dice(msg, args, comment);
    }

    if (param_name == "cs" || param_name == "create_sheet") {
        create_sheet(msg, args, comment);
    }
}

//
// Start the bot.
//
export function start_bot() {
    const client = new Discord.Client();

    client.on("ready", () => {
        if (client.user) {
            console.log(`Logged in as ${client.user.tag}!`);
        }
    });

    client.on("message", msg => {
        if (msg.author.bot || msg.content.length == 0 || msg.content[0] != "/") {
            return;
        }

        calculate_args(msg);
    });

    if(process.env.DISCORD_BOT_TOKEN == undefined) {
        console.log("please set ENV: DISCORD_BOT_TOKEN");
        process.exit(0);
    }

    client.login(process.env.DISCORD_BOT_TOKEN);
}
