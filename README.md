# Unofficial Cthulhu Bot

[![GitHub issues](https://img.shields.io/github/issues/capra314cabra/cthulhu_bot)](https://github.com/capra314cabra/cthulhu_bot/issues)
[![License](https://img.shields.io/github/license/capra314cabra/cthulhu_bot)](https://github.com/capra314cabra/cthulhu_bot/blob/master/LICENSE)

<p align="center">
    <img width="500" height="400" src="https://github.com/capra314cabra/cthulhu_bot/blob/master/img/icon.png">
</p>

An unofficial Discord bot which helps you to play Cthulhu TRPG.

Please note that **Chaosium Inc. owns the copyright** of Cthulhu TRPG,
and this bot contains nothing related to data in the CoC book.
Buy the CoC book first.

[Invite this bot!](https://discord.com/api/oauth2/authorize?client_id=811123481370558505&permissions=256064&scope=bot)

## Examples

<p align="center">
    <img width="462" height="120" src="https://github.com/capra314cabra/cthulhu_bot/blob/master/img/example1.png">
</p>

## Commands available

- `<arg>` : An argument required.
- `[arg]` : An argument not required.

|Command|Description|
|:---|:---|
|`/roll [limit] #[Comment]`|Roll a `d100` dice and judge the result whether it is succeeded or not.|
|`/r [limit] #[Comment]`|Equal to `/roll`.|
|`/custom_roll <expression> #[Comment]`|Evaluate the expression given. Use [dice-expression-evaluator](https://github.com/dbkang/dice-expression-evaluator) internally.|
|`/cr <expression> #[Comment]`|Equal to `/custom_roll`.|

## Memo

Inspired by [Sidekick](https://github.com/ArtemGr/Sidekick), which is no longer available.
