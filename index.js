const Discord = require("discord.js");
const client = new Discord.Client();

client.on("ready", () => {
    console.log(`Logged in as ${client.user.tag}!`);
});

client.on("message", msg => {
    if (msg.content === "ping") {
        msg.reply("pong");
    }
});

if(process.env.DISCORD_BOT_TOKEN == undefined) {
    console.log("please set ENV: DISCORD_BOT_TOKEN");
    process.exit(0);
}

client.login( process.env.DISCORD_BOT_TOKEN );
