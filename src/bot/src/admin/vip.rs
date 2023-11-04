use std::fmt::format;
use std::num::ParseIntError;
use std::process::exit;
use std::str::FromStr;
use teloxide::{Bot, dptree};
use teloxide::dispatching::{HandlerExt, UpdateFilterExt, UpdateHandler};
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dptree::{case, entry};
use teloxide::payloads::SendMessageSetters;
use teloxide::payloads::{CopyMessageSetters, SendDiceSetters};
use teloxide::prelude::{CallbackQuery, Dialogue, Dispatcher, Requester};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, Message, Update};
use teloxide::utils::command::BotCommands;
use crate::admin::types::{AddNewVipState, HandlerResult, MyDialogue, State};
use crate::db;
use crate::db::models;

pub async fn vip(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    let actions = ["Добавить пользователя", "Удалить пользователя"]
        .iter()
        .enumerate()
        .map(|(index, action)|
            InlineKeyboardButton::callback(*action, index.to_string()));

    dialogue.update(State::Vip).await?;

    bot.send_message(msg.chat.id, "VIP меню. Выберите действие:")
        .reply_markup(InlineKeyboardMarkup::new([actions]))
        .await?;
    Ok(())
}

pub async fn add_new_vip(bot: Bot, msg: Message, dialogue: MyDialogue, state: AddNewVipState) -> HandlerResult {
    match state {
        AddNewVipState::AwaitId => {
            match msg.text() {
                Some(text) => {
                    let id = match isize::from_str(text) {
                        Ok(id) => {id}
                        Err(_) => {
                            bot.send_message(msg.chat.id, "Вы ввели не число, попробуйте заново").await?;
                            return Ok(())
                        }
                    };
                    dialogue.update(State::AddNewVip {state: AddNewVipState::AwaitName { id }}).await?;
                    bot.send_message(msg.chat.id, "Напишите, ассоциацию для этого пользователя. \
            Это будет храниться в базе данных и если вы будите удалять пользователя - \
            вы сможете его найти по этого имени").await?;
                }
                None => {
                    bot.send_message(msg.chat.id, "Не тот тип данных. \
                    Напишите ID (к примеру: \"11381237213271\")").await?;
                }
            }
        }
        AddNewVipState::AwaitName {id} => {
            match msg.text() {
                Some(text) => {
                    bot.send_message(msg.chat.id, "Записавыем в базу данных...").await?;
                    let mut users = db::get_users().await;
                    users.push(models::User {
                        tg_id: id,
                        name: text.to_string(),
                    } );
                    db::flush_users(users).await;
                    bot.send_message(msg.chat.id, "Успешно. Выходим в главное меню, \
                    можете написать /start, чтобы продолжить!").await?;
                },
                None => {
                    bot.send_message(msg.chat.id, "Вы отправили не текст.").await?;
                }
            }
        }
    };
    Ok(())
}

pub async fn remove_vip(bot: Bot, callback: CallbackQuery, dialogue: MyDialogue) -> HandlerResult {
    let msg = callback.message.expect("Message from callback wasn't received");
    let data_option = callback.data.expect("Data from vip callback wasn't received");
    let data = data_option.as_str();

    match isize::from_str(data) {
        Ok(i) => {
            let mut users = db::get_users().await;
            users = users.into_iter()
                .filter(|user| user.tg_id != i)
                .collect();
            db::flush_users(users).await;
            bot.send_message(msg.chat.id, "Успешно!").await?;
        }
        Err(e) => {
            bot.send_message(msg.chat.id, "Что-то пошло не так... ").await?;
        }
    }
    bot.answer_callback_query(callback.id).await?;

    Ok(())
}

pub async fn callback_vip(bot: Bot, callback: CallbackQuery, dialogue: MyDialogue) -> HandlerResult {
    let msg = callback.message.expect("Message from callback wasn't received");

    match callback.data.expect("Data from vip callback wasn't received").as_str() {
        "0" => {
            dialogue.update(State::AddNewVip {state: AddNewVipState::AwaitId}).await?;
            bot.send_message(msg.chat.id, "Пришлите ID пользователя, который вы хотите добавить. \
            Получить ID можно в этом боте - @userdatailsbot")
                .await?;
        },
        "1" => {
            dialogue.update(State::RemoveVip).await?;
            let users = db::get_users().await;
            for user in &users {
                let markup = InlineKeyboardMarkup::new([
                    [InlineKeyboardButton::callback("Удалить", user.tg_id.to_string())]
                ]);
                bot.send_message(msg.chat.id, &user.name).reply_markup(markup).await?;
            }
            dialogue.update(State::RemoveVip).await?;
            let text = format!("Показанно {} пользователей с VIP статусом. \
            Выберите пользователся из списка и нажмите на кнопку справа! \
            Когда вы захотите выйти из диалога напишите /exit", users.len());
            bot.send_message(msg.chat.id, text)
                .await?;
        },
        other => {
            bot.send_message(msg.chat.id, format!("Что-то пошло нет так... ({})", other))
                .await?;
        }
    }
    bot.answer_callback_query(callback.id).await?;

    Ok(())
}