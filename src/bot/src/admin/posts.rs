use teloxide::Bot;
use teloxide::prelude::CallbackQuery;
use crate::admin::types::{HandlerResult, MyDialogue};
use std::str::FromStr;
use teloxide::payloads::SendMessageSetters;
use teloxide::payloads::{CopyMessageSetters, SendDiceSetters};
use teloxide::prelude::{Requester};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, Message};
use uuid::Uuid;
use crate::admin::media_handler::{add_document, add_video};
use crate::admin::types::State::{AwaitPostMedia, AwaitPostMediaType, AwaitPostName, AwaitPostStatus};
use crate::db;
use crate::db::models::{MediaType, Model, Post};

pub async fn await_post_action(bot: Bot, callback: CallbackQuery,
                               dialogue: MyDialogue, model: Model) -> HandlerResult {
    let msg = callback.message.expect("Message from callback wasn't received");
    let data_option = callback.data.expect("Data from vip callback wasn't received");
    let data = data_option.as_str();

    match data {
        "create_post" => {
            dialogue.update(AwaitPostName {model}).await?;
            bot.send_message(msg.chat.id, "Напишите имя для поста:").await?;
        }
        other => {
            let mut model = model;
            let mut posts = model.posts.clone();
            posts = posts.into_iter()
                .filter(|current| current.id.to_string() != other).collect();

            let mut models = db::get_models().await;
            models = models.into_iter()
                .filter(|current| current.id != model.id).collect();

            model.posts = posts;
            models.push(model);
            db::flush_models(models).await;

            dialogue.exit().await?;
            bot.send_message(msg.chat.id, "Успешно! Вышли в главное меню \
                    (/start)").await?;
        }
    }
    bot.answer_callback_query(callback.id).await?;
    Ok(())
}

pub async fn await_post_name(bot: Bot, msg: Message,
                             dialogue: MyDialogue, model: Model) -> HandlerResult {
    match msg.text() {
        Some(text) => {
            dialogue.update(AwaitPostStatus { model, name: text.to_string()}).await?;
            let markup = InlineKeyboardMarkup::new([
                [InlineKeyboardButton::callback("VIP", "true"),
                    InlineKeyboardButton::callback("Не VIP", "false")]
            ]);
            bot.send_message(msg.chat.id, "Выберите статус:")
                .reply_markup(markup).await?;
        },
        None => {
            bot.send_message(msg.chat.id, "Имя должно быть в виде текста!").await?;
        }
    }
    Ok(())
}

pub async fn await_post_status(bot: Bot, callback: CallbackQuery, dialogue: MyDialogue,
                               (model, name): (Model, String)) -> HandlerResult {
    let msg = callback.message.expect("Message from callback wasn't received");
    let data_option = callback.data.expect("Data from vip callback wasn't received");
    let data = data_option.as_str();

    match bool::from_str(data) {
        Ok(status) => {
            dialogue.update(AwaitPostMediaType {
                model,
                name,
                status,
            }).await?;
            let markup = InlineKeyboardMarkup::new([
                [
                    InlineKeyboardButton::callback("Изображение", "0"),
                    InlineKeyboardButton::callback("Видео", "1")
                ]
            ]);
            bot.send_message(msg.chat.id, "Отлично. Выберите тип поста:")
                .reply_markup(markup).await?;
        },
        Err(_) => {
            bot.send_message(msg.chat.id, "Возможно вы нажали на старую кнопку. \
                    Нажмите на статус поста!").await?;
        }
    }
    bot.answer_callback_query(callback.id).await?;
    Ok(())
}

pub async fn await_post_media_type(bot: Bot, callback: CallbackQuery, dialogue: MyDialogue,
                                   (model, name, status): (Model, String, bool)) -> HandlerResult {
    let msg = callback.message.expect("Message from callback wasn't received");
    let data_option = callback.data.expect("Data from vip callback wasn't received");
    let data = data_option.as_str();

    let media_type = match data {
        "1" => MediaType::Video,
        _ => MediaType::Image
    };
    dialogue.update(AwaitPostMedia {
        model,
        name,
        status,
        media_type,
    }).await?;
    bot.send_message(msg.chat.id, "Отлично. Скиньте документ с медиа!").await?;
    bot.answer_callback_query(callback.id).await?;
    Ok(())
}

pub async fn await_post_media(bot: Bot, msg: Message, dialogue: MyDialogue,
                              (model, name, status, media_type): (Model, String, bool, MediaType)) -> HandlerResult {
    match media_type {
        MediaType::Image => {
            match msg.document() {
                Some(document) => {
                    let mut model = model;
                    let post = Post {
                        id: Uuid::new_v4(),
                        name,
                        media: add_document(document, &bot, media_type).await
                            .expect("Can't change document"),
                        is_vip: status,
                    };
                    model.posts.push(post);
                    let mut models = db::get_models().await;
                    models = models.into_iter()
                        .filter(|current| current.id != model.id).collect();
                    models.push(model);
                    db::flush_models(models).await;
                    dialogue.exit().await?;
                    bot.send_message(msg.chat.id, "Успешно, выходим в главное меню (/start)")
                        .await?;
                },
                None => {
                    bot.send_message(msg.chat.id, "Не тот тип данных!").await?;
                }
            }
        },
        MediaType::Video => {
            match msg.video() {
                Some(video) => {
                    let mut model = model;
                    let post = Post {
                        id: Uuid::new_v4(),
                        name,
                        media: add_video(video, &bot, media_type).await
                            .expect("Can't change document"),
                        is_vip: status,
                    };
                    model.posts.push(post);
                    let mut models = db::get_models().await;
                    models = models.into_iter()
                        .filter(|current| current.id != model.id).collect();
                    models.push(model);
                    db::flush_models(models).await;
                    dialogue.exit().await?;
                    bot.send_message(msg.chat.id, "Успешно, выходим в главное меню (/start)")
                        .await?;
                },
                None => {
                    bot.send_message(msg.chat.id, "Не тот тип данных!").await?;
                }
            }
        }
    }
    Ok(())
}