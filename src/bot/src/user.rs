use std::ops::Add;
use std::str::FromStr;
use std::thread;
use teloxide::Bot;
use teloxide::prelude::{Requester, ResponseResult};
use teloxide::{prelude::*, utils::command::BotCommands};
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dptree::case;
use teloxide::types::{ButtonRequest, Currency, InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton, KeyboardMarkup, LabeledPrice, Message, OrderInfo, WebAppInfo};
use crate::db;
use crate::db::models;
use crate::db::models::User;

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
pub async fn run() {
    // pretty_env_logger::init();

    let token = "6463143170:AAGevkCPBTaAEEnKmtLCYzvxHpdjhzsHJe4";
    let bot = Bot::new(token);

    let filter_command = teloxide::filter_command::<Command, _>()
        .branch(case![Command::Help].endpoint(help))
        .branch(case![Command::Start].endpoint(start))
        .branch(case![Command::Vip].endpoint(vip));

    let filter_message = Update::filter_message()
        .branch(filter_command)
        .branch(dptree::endpoint(invalid));

    let filter_pre_checkout_query = Update::filter_pre_checkout_query()
        .branch(dptree::endpoint(pre_checkout_query));

    let handler = dptree::entry()
        .branch(filter_message)
        .branch(filter_pre_checkout_query);

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
        url: "https://id.tinkoff.ru/auth/step?cid=PMZrkDhN16O0".parse().unwrap()
    };

    let list = InlineKeyboardMarkup::new(vec![vec![
        InlineKeyboardButton::web_app("Открыть", webapp)
    ]]);

    bot.send_message(msg.chat.id, text).reply_markup(list).await?;
    Ok(())
}

async fn pre_checkout_query(bot: Bot, query: PreCheckoutQuery) -> HandlerResult {
    match i64::from_str(query.invoice_payload.as_str()) {
        Ok(id) => {
            let chat_id = ChatId(id);
            bot.send_message(chat_id, "Начинаем запись пользователя...").await?;

            let mut users = db::get_users().await;
            users.push(User {
                tg_id: isize::from_str(query.from.id.to_string().as_str()).unwrap(),
                name: query.from.first_name,
            } );
            db::flush_users(users).await;

            bot.answer_pre_checkout_query(query.id, true).await?;
            bot.send_message(chat_id, "Успешно. Завершаем покупку!").await?;
        },
        Err(_) => {
            bot.answer_pre_checkout_query(query.id, false).await?;
        }
    }
    Ok(())
}


async fn vip(bot: Bot, msg: Message) -> HandlerResult {
    let users = db::get_users().await;
    let current_user = msg.from().expect("User from VIP command wasn't received");

    let users: Vec<User> = users.into_iter()
        .filter(|user| user.tg_id.to_string() == current_user.id.to_string())
        .collect();

    match users.first() {
        Some(_) => {
            bot.send_message(msg.chat.id, "Вы уже VIP, спасибо за поддержку нашего сервиса!").await?;
        },
        None => {
            bot.send_message(msg.chat.id, "Хотите вип?").await?;
            bot.send_invoice(
                msg.chat.id,
                "VIP-статус",
                "Доступ большому числу эксклюзивного контента",
                msg.chat.id.to_string(),
                "381764678:TEST:70885",
                "RUB", // currency code
                vec![LabeledPrice::new("VIP", 29999)],
            ).send().await?;
        }
    }
    Ok(())
}

async fn invalid(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Ваша комманда не найдена. Начните /start или /help").await?;
    Ok(())
}