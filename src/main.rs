#[macro_use]
extern crate diesel;

extern crate dotenv;
mod db;
mod bot;

use std::thread;
use teloxide::Bot;
use teloxide::prelude::{Requester, ResponseResult};
use teloxide::{prelude::*, utils::command::BotCommands};
use teloxide::dptree::case;
use teloxide::types::{ButtonRequest, InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton, KeyboardMarkup, Message, WebAppInfo};

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Существуют эти комманды:")]
enum Command {
    #[command(description = "вы тут")]
    Help,
    #[command(description = "получитить основную информацию")]
    Start,
    #[command(description = "получить VIP достук к боту")]
    Vip,
}

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    thread::spawn(bot::start);

    let bot = Bot::from_env();

    let filter_command = teloxide::filter_command::<Command, _>()
        .branch(case![Command::Help].endpoint(help))
        .branch(case![Command::Start].endpoint(start))
        .branch(case![Command::Vip].endpoint(vip));

    let filter_message = Update::filter_message()
        .branch(filter_command)
        .branch(dptree::endpoint(invalid));

    let handler = dptree::entry()
        .branch(filter_message);

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn help(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
    Ok(())
}

async fn start(bot: Bot, msg: Message) -> HandlerResult {
    let text = format!("
            1000+ Бесплатных СЛИВОВ\n\
            / ТУТ БУДЕТ ССЫЛКА /\n\
            ТАКЖЕ СМОТРИ СЛИВЫ ПРЯМО В БОТЕ\n\
            Кликай на кнопку 👇👇👇");

    let webapp = WebAppInfo {
        url: "http://example.com/".parse().unwrap()
    };

    let list = InlineKeyboardMarkup::new(vec![vec![
        InlineKeyboardButton::web_app("Открыть", webapp)
    ]]);

    bot.send_message(msg.chat.id, text).reply_markup(list).await?;
    Ok(())
}

async fn vip(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Тут будет реализованна оплата по карте").await?;
    Ok(())
}

async fn invalid(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Ваша комманда не найдена. Начните /start или /help").await?;
    Ok(())
}