import * as Discord from "discord.js";
import * as http from "http";

const client = new Discord.Client();

client.on("ready", () => {
    if (client.user) {
        console.log(`Logged in as ${client.user.tag}!`);
    }
});

client.on("message", msg => {
    if (msg.author.bot) {
        return;
    }
    if (msg.content === "ping") {
        msg.reply("pong");
    }
});

if(process.env.DISCORD_BOT_TOKEN == undefined) {
    console.log("please set ENV: DISCORD_BOT_TOKEN");
    process.exit(0);
}

client.login(process.env.DISCORD_BOT_TOKEN);

//
// A logic to make this bot awake.
//
const server = http.createServer((req, res) => {
    res.writeHead(200);
    res.end('ok');
});

server.listen(3000);
