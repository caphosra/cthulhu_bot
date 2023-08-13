# Unofficial Cthulhu Bot

[![Test](https://github.com/caphosra/cthulhu_bot/actions/workflows/test.yml/badge.svg)](https://github.com/caphosra/cthulhu_bot/actions/workflows/test.yml)
[![Build](https://github.com/caphosra/cthulhu_bot/actions/workflows/build.yml/badge.svg)](https://github.com/caphosra/cthulhu_bot/actions/workflows/build.yml)
[![GitHub issues](https://img.shields.io/github/issues/caphosra/cthulhu_bot)](https://github.com/caphosra/cthulhu_bot/issues)

<p align="center">
    <img width="500" height="400" src="https://raw.githubusercontent.com/caphosra/cthulhu_bot/master/img/icon.png">
</p>

This bot emulates dice rolling. It is tuned for CoC, but you can use this for general purposes.

**Note that  Chaosium Inc. owns the copyright of Call of Cthulhu.**

[Invite this bot!](https://discord.com/api/oauth2/authorize?client_id=811123481370558505&permissions=277025572928&scope=bot%20applications.commands)

## Commands available

- `<arg>` : An argument required.
- `[arg]` : An argument not required.

The bot hosted by the owner is DB-free. If you want to use not DB-free features, you should host this bot by yourself.

|Command|DB-free|Description|
|:---|:---:|:---|
|`/choose <choice A, choice B, ...>`|:white_check_mark:|Makes a random choice.|
|`/cs`|:white_check_mark:|Creates a character sheet.|
|`/roll <dice> [comment]`|:white_check_mark:|Rolls designated dices. Expressions supported by [d20](https://github.com/pholactery/d20) can be used.|
|`/set <param> <value>`||Assigns a value to your parameter.|
|`/skill <value> [comment]`|:white_check_mark:|Attempts a skill roll. In other words, rolls 1d100.|
|`/status`||Displays your status.|

### Roll dices

Command: `/roll` dice:`3d5 + 2d4`

<p align="center">
    <img width="250" height="122" src="https://raw.githubusercontent.com/caphosra/cthulhu_bot/master/img/01.png">
</p>

### Attempts a skill roll.

Command: `/skill` value:`50` comment:`Listen`

<p align="center">
    <img width="250" height="120" src="https://raw.githubusercontent.com/caphosra/cthulhu_bot/master/img/02.png">
</p>

### Create a character sheet

Command: `/cs`

<p align="center">
    <img width="250" height="171" src="https://raw.githubusercontent.com/caphosra/cthulhu_bot/master/img/03.png">
</p>

## Memo

Inspired by [Sidekick](https://github.com/ArtemGr/Sidekick), which is no longer available.
