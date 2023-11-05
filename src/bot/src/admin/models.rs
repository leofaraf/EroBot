use std::error::Error;
use std::ffi::OsStr;
use std::fmt::format;
use std::num::ParseIntError;
use std::ops::Add;
use std::path::Path;
use std::process::exit;
use std::str::{FromStr, Split};
use teloxide::{Bot, dptree};
use teloxide::dispatching::{HandlerExt, UpdateFilterExt, UpdateHandler};
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dptree::{case, entry};
use teloxide::net::Download;
use teloxide::payloads::SendMessageSetters;
use teloxide::payloads::{CopyMessageSetters, SendDiceSetters};
use teloxide::prelude::{CallbackQuery, Dialogue, Dispatcher, Requester};
use teloxide::types::{Document, InlineKeyboardButton, InlineKeyboardMarkup, Message, Update};
use teloxide::utils::command::BotCommands;
use tokio::fs::File;
use uuid::Uuid;
use crate::admin::types::{AddNewModelState, HandlerResult, MyDialogue, State};
use crate::admin::types::State::AddNewModel;
use crate::db;
use crate::db::models::{Media, MediaType, Model};

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

pub async fn callback_change_models(bot: Bot, callback: CallbackQuery, dialogue: MyDialogue) -> HandlerResult {
    let msg = callback.message.expect("Message from callback wasn't received");
    let data_option = callback.data.expect("Data from vip callback wasn't received");
    let data = data_option.as_str();

    match data {
        "delete" => {

        },
        "posts" => {

        },
        "change_name" => {

        },
        "change_photo" => {

        }
        _ => {
            bot.send_message(msg.chat.id,
                             "Что-то пошло не так... Возможно вы нажали не на ту кнопку").await?;
        }
    }
    bot.answer_callback_query(callback.id).await?;

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
            let actions = InlineKeyboardMarkup::new([[
                InlineKeyboardButton::callback("Удалить", "delete"),
                InlineKeyboardButton::callback("Перейти к постам", "posts")
            ],[
                InlineKeyboardButton::callback("Изменить имя", "change_name"),
                InlineKeyboardButton::callback("Изменить фото", "change_photo")
            ]]);
            dialogue.update()
            bot.send_message(msg.chat.id, "Хорошо, что вы хотите сделать с моделью?")
                .reply_markup(actions).await?;
        }
    }
    bot.answer_callback_query(callback.id).await?;

    Ok(())
}

pub async fn add_new_model(bot: Bot, msg: Message, dialogue: MyDialogue, state: AddNewModelState)
    -> HandlerResult {
    let mut state = state;

    match state {
        AddNewModelState::ReceiveModelName => {
            match msg.text() {
                Some(text) => {
                    state = AddNewModelState::ReceiveModelPhoto {name: text.to_string()};
                    dialogue.update(AddNewModel {state}).await?;
                    bot.send_message(msg.chat.id, "Отлично, пришлите фото с для модели!").await?;
                },
                None => {
                    bot.send_message(msg.chat.id, "Нужно ввести имя в виде текста!").await?;
                }
            }
        },
        AddNewModelState::ReceiveModelPhoto {name} => {
            match msg.document() {
                Some(document) => {
                    let media = add_document(document, &bot).await.unwrap();
                    let mut models = db::get_models().await;
                    models.push(Model {
                        id: Uuid::new_v4(),
                        name,
                        media,
                        posts: vec![],
                    });
                    db::flush_models(models).await;
                    dialogue.exit().await?;
                    bot.send_message(msg.chat.id, "Отлично, создали, вышли \
                    в главное меню /start!").await?;

                },
                None => {
                    bot.send_message(msg.chat.id, "Не тот тип данных!").await?;
                }
            }
        }
    }

    Ok(())
}

async fn add_document(document: &Document, bot: &Bot) -> Result<Media, ()> {
    let name = document.file_name.clone().unwrap();

    let extension = match Path::new(name.as_str()).extension() {
        None => None,
        Some(path) => path.to_str()
    };

    let media = Media {
        path: match extension {
            Some(text) => Uuid::new_v4().to_string().add(".").add(text),
            None => Uuid::new_v4().to_string()
        },
        media_type: MediaType::Image
    };

    let file =  bot.get_file(&document.file.id).await.unwrap();
    let mut dst = File::create(&media.path).await.unwrap();

    // Download the voice message and write it to the file
    bot.download_file(&file.path, &mut dst).await.unwrap();
    Ok((media))
}

async fn delete_document(media: Media) {

}