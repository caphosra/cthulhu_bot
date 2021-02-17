import * as Discord from "discord.js";
import DiceExpression from "dice-expression-evaluator";

//
// Roll a d100 dice.
//
function roll_d100(msg: Discord.Message, args: string, comment: string) {
    const number_regex = /\d+/;
    const matched_number = args.match(number_regex);
    let number = -1;
    if (matched_number) {
        number = parseInt(matched_number[0]);
    }

    const expr = new DiceExpression("d100");
    let result = expr.roll().roll;

    let result_message = "";
    if (result <= 5) {
        result_message = `:crown: Critical! (${result})`
    }
    else if (result >= 95) {
        result_message = `:skull: Fumble! (${result})`
    }
    else if (number == -1) {
        result_message = `:question: Can't judge. (${result})`
    }
    else if (result <= number) {
        result_message = `:o: Success (${result} <= ${number})`
    }
    else {
        result_message = `:x: Failed (${result} > ${number})`
    }

    if (comment != "") {
        comment = `\`${comment}\``
    }
    msg.reply(`${comment} Result : ${result_message}`)
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
