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
use crate::admin::types::{AddNewModelState, HandlerResult, MyDialogue, State};
use crate::admin::types::State::AddNewModel;
use crate::db;

pub async fn models(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    let models = db::get_models().await;
    dialogue.update(State::Models).await?;
    for model in models {
        let text = format!("Имя: {} \n\
        Кол-во постов: {}", model.name, model.posts.len());
        let markup = InlineKeyboardMarkup::new([
            [InlineKeyboardButton::callback("Перейти", model.id.to_string())]
        ]);
        bot.send_message(msg.chat.id, text).reply_markup(markup).await?;
    }
    let markup = InlineKeyboardMarkup::new([
        [InlineKeyboardButton::callback("Создать новую модель", "create")]
    ]);
    bot.send_message(msg.chat.id, "Нажмите на кнопку ниже если хотите создать новую модель. \
    Если хотите выйти в любой момент из диалога - /exit").reply_markup(markup).await?;
    Ok(())
}

pub async fn callback_models(bot: Bot, callback: CallbackQuery, dialogue: MyDialogue) -> HandlerResult {
    let msg = callback.message.expect("Message from callback wasn't received");
    let data_option = callback.data.expect("Data from vip callback wasn't received");
    let data = data_option.as_str();

    match data {
        "create" => {
            dialogue.update(AddNewModel {state: AddNewModelState::ReceiveModelName}).await?;
            bot.send_message(msg.chat.id, "Начнем создние модели, напиши ее имя:").await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Хорошо, что вы хотите сделать с моделью?").await?;
        }
    }

    Ok(())
}

pub async fn add_new_model(bot: Bot, msg: Message, dialogue: MyDialogue, state: AddNewModelState) -> HandlerResult {
    let mut state = state;

    match state {
        AddNewModelState::ReceiveModelName => {
            match msg.text() {
                Some(text) => {
                    state = AddNewModelState::ReceiveModelPhoto {name: text.to_string()};
                    dialogue.update(AddNewModel {state});
                    bot.send_message(msg.chat.id, "Отлично, пришлите фото с для модели!").await?;
                },
                None => {
                    bot.send_message(msg.chat.id, "Нужно ввести имя в виде текста!").await?;
                }
            }
        },
        AddNewModelState::ReceiveModelPhoto {name} => {
            match msg.photo() {
                Some(photo) => {
                    todo!("реализовать хранилище фото, сделаю точно после прогулки))) \n\
                    все они должны быть долступны по адресу http://ip/media/*имя_файла*")
                },
                None => {
                    bot.send_message(msg.chat.id, "Не тот тип данных!").await?;
                }
            }
        }
    }

    Ok(())
}