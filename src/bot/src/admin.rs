mod types;
mod vip;
mod models;
mod posts;
mod media_handler;

use std::fmt::format;
use std::num::ParseIntError;
use std::process::exit;
use std::str::FromStr;
use dotenv::dotenv;
use dotenv_codegen::dotenv;
use teloxide::{Bot, dptree};
use teloxide::dispatching::{HandlerExt, UpdateFilterExt, UpdateHandler};
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dptree::{case, entry};
use teloxide::payloads::SendMessageSetters;
use teloxide::payloads::{CopyMessageSetters, SendDiceSetters};
use teloxide::prelude::{CallbackQuery, Dialogue, Dispatcher, Requester};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, Message, Update};
use teloxide::utils::command::BotCommands;
use crate::admin::models::{add_new_model, callback_add_new_model, callback_change_models, callback_models, change_model, models};
use crate::admin::posts::{await_post_action, await_post_media, await_post_media_type, await_post_name, await_post_status};
use crate::admin::types::{AddNewVipState, Command, HandlerResult, MyDialogue, State};
use crate::admin::vip::{add_new_vip, callback_vip, remove_vip, vip};

use crate::db;

#[tokio::main]
pub async fn run() {
    pretty_env_logger::init();
    dotenv().ok();

    let bot = Bot::new(
        dotenv!("ADMIN_BOT_TOKEN"));

    Dispatcher::builder(bot, schema())
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    let filter_command = teloxide::filter_command::<Command, _>()
        .branch(case![Command::Start].endpoint(start))
        .branch(case![Command::Vip].endpoint(vip))
        .branch(case![Command::Models].endpoint(models))
        .branch(case![Command::Exit].endpoint(exit_dialogue));

    let posts_handler = entry()
        .branch(case![State::AwaitPostAction {model}].endpoint(await_post_action))
        .branch(case![State::AwaitPostMediaType {model, name, status}].endpoint(await_post_media_type));

    let message_handler = Update::filter_message()
        .enter_dialogue::<Message, InMemStorage<State>, State>()
        .branch(filter_command)
        .branch(case![State::AwaitPostName {model}].endpoint(await_post_name))
        .branch(case![State::AwaitPostMedia {model, name, status, media_type}].endpoint(await_post_media))
        .branch(case![State::AddNewVip {state}].endpoint(add_new_vip))
        .branch(case![State::AddNewModel {state}].endpoint(add_new_model))
        .branch(case![State::ChangeModel {state}].endpoint(change_model))
        .branch(entry().endpoint(invalid_point));

    let callback_query_handler = Update::filter_callback_query()
        .enter_dialogue::<CallbackQuery, InMemStorage<State>, State>()
        .branch(posts_handler)
        .branch(case![State::AwaitPostStatus {model, name}].endpoint(await_post_status))
        .branch(case![State::AddNewModel {state}].endpoint(callback_add_new_model))
        .branch(case![State::Vip].endpoint(callback_vip))
        .branch(case![State::RemoveVip].endpoint(remove_vip))
        .branch(case![State::Models].endpoint(callback_models))
        .branch(case![State::ChangeModel {state}].endpoint(callback_change_models));

    entry()
        .branch(callback_query_handler)
        .branch(message_handler)
}

async fn exit_dialogue(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    dialogue.exit().await?;
    bot.send_message(
        msg.chat.id,
        "Вышли. Напишите камманду, \
         которую вы хотите сделать или получите информацию о них при помощи /start")
        .await?;
    Ok(())
}

async fn start(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(
        msg.chat.id,
        Command::descriptions().to_string()).await?;
    Ok(())
}

async fn invalid_point(bot: Bot, msg: Message) -> HandlerResult {
    let text = "Неизвестная комманда. \
     Если вы не можете определиться, что вы хотите сделать напишите /start, \
     если хотите выйти из прошлого диалога - напишите /exit";

    bot.send_message(msg.chat.id, text).await?;

    Ok(())
}