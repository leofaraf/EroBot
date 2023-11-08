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
use teloxide::types::{Document, InlineKeyboardButton, InlineKeyboardMarkup, Me, Message, Update, Video};
use teloxide::utils::command::BotCommands;
use tokio::fs::{create_dir_all, File};
use uuid::Uuid;
use crate::admin::types::{AddNewModelState, ChangeModelState, ChangeModelType, HandlerResult,
                          MyDialogue, State};
use crate::admin::types::ChangeModelType::AwaitPostMediaType;
use crate::admin::types::State::{AddNewModel, ChangeModel};
use crate::db;
use crate::db::models::{Media, MediaType, Model, ModelCategory, Post};

pub async fn models(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    let models = db::get_models().await;
    dialogue.update(State::Models).await?;
    for model in models {
        let text = format!("Имя: {} \n\
        Категория: {} \n\
        Кол-во постов: {}", model.name, serde_json::to_string(&model.category)
            .unwrap_or("".to_string()), model.posts.len());
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

pub async fn callback_change_models(bot: Bot, callback: CallbackQuery,
                                    dialogue: MyDialogue, state: ChangeModelState) -> HandlerResult {
    let msg = callback.message.expect("Message from callback wasn't received");
    let data_option = callback.data.expect("Data from vip callback wasn't received");
    let data = data_option.as_str();

    match state.type_of_await {
        ChangeModelType::AwaitPostStatus {name} => {
            match bool::from_str(data) {
                Ok(status) => {
                    dialogue.update(ChangeModel { state: ChangeModelState {
                        model: state.model,
                        type_of_await: ChangeModelType::AwaitPostMediaType {name, status},
                    }}).await?;
                    let markup = InlineKeyboardMarkup::new([
                        [
                            InlineKeyboardButton::callback("Изображение", "0"),
                            InlineKeyboardButton::callback("Видео", "1")
                        ]
                    ]);
                    bot.send_message(msg.chat.id, "Отлично. Выберите тип поста:")
                        .reply_markup(markup).await?;
                },
                Err(e) => {
                    bot.send_message(msg.chat.id, "Возможно вы нажали на старую кнопку. \
                    Нажмите на статус поста!").await?;
                }
            }
        },
        AwaitPostMediaType {name, status } => {
            let media_type = match data {
                "1" => MediaType::Video,
                _ => MediaType::Image
            };
            dialogue.update(ChangeModel { state: ChangeModelState {
                model: state.model,
                type_of_await: ChangeModelType::AwaitPostMedia {name, status, media_type},
            }}).await?;
            bot.send_message(msg.chat.id, "Отлично. Скиньте документ с медиа!").await?;
        }
        ChangeModelType::AwaitCategory => {
            let mut model = state.model;
            model.category = match data {
                "1" => ModelCategory::Boosty,
                "2" => ModelCategory::Star,
                "3" => ModelCategory::Tiktok,
                "4" => ModelCategory::OnlyFans,
                "5" => ModelCategory::Twitch,
                _ => ModelCategory::Influential
            };
            let mut models = db::get_models().await;
            models = models.into_iter()
                .filter(|current| current.id != model.id).collect();
            models.push(model);
            db::flush_models(models).await;
            bot.send_message(msg.chat.id, "Успешно, выходим в главное меню (/start)").await?;
            dialogue.exit().await?;
        }
        ChangeModelType::AwaitPostAction => {
            match data {
                "create_post" => {
                    dialogue.update(ChangeModel { state: ChangeModelState {
                        model: state.model,
                        type_of_await: ChangeModelType::AwaitPostName,
                    }}).await?;
                    bot.send_message(msg.chat.id, "Напишите имя для поста:").await?;
                }
                other => {
                    let mut model = state.model;
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
        },
        _ => {
            match data {
                "delete" => {
                    let mut models = db::get_models().await;
                    models = models.into_iter()
                        .filter(|current| current.id != state.model.id).collect();
                    db::flush_models(models).await;
                    bot.send_message(msg.chat.id, "Успешно, выходим в главное меню! (/start)").await?;
                    dialogue.exit().await?;
                },
                "posts" => {
                    for post in &state.model.posts {
                        let delete_btn = InlineKeyboardMarkup::new([
                            [InlineKeyboardButton::callback("Удалить", post.id.to_string())]
                        ]);
                        bot.send_message(msg.chat.id, format!("Название: {}, \n\
                        Вип: {}", post.name, post.is_vip.to_string())).reply_markup(delete_btn).await?;
                    }
                    let create_btn = InlineKeyboardMarkup::new([
                        [InlineKeyboardButton::callback("Создать новый", "create_post")]
                    ]);
                    let text = format!("Показанно {} постов по модели. \
            Выберите пост из списка и нажмите на кнопку справа если хотите удалить! \
            Когда вы захотите выйти из диалога напишите /exit", &state.model.posts.len());
                    dialogue.update(ChangeModel { state: ChangeModelState {
                        model: state.model,
                        type_of_await: ChangeModelType::AwaitPostAction,
                    }}).await?;
                    bot.send_message(msg.chat.id, text).reply_markup(create_btn).await?;
                },
                "change_name" => {
                    dialogue.update(ChangeModel { state: ChangeModelState {
                        model: state.model,
                        type_of_await: ChangeModelType::AwaitName,
                    }}).await?;
                    bot.send_message(msg.chat.id, "Напишите новое имя для этой модели:").await?;
                },
                "change_photo" => {
                    dialogue.update(ChangeModel { state: ChangeModelState {
                        model: state.model,
                        type_of_await: ChangeModelType::AwaitPhoto,
                    }}).await?;
                    bot.send_message(msg.chat.id, "Пришлите фото:").await?;
                },
                "change_category" => {
                    dialogue.update(ChangeModel { state: ChangeModelState {
                        model: state.model,
                        type_of_await: ChangeModelType::AwaitCategory,
                    }}).await?;
                    bot.send_message(msg.chat.id, "Выберите категорию:")
                        .reply_markup(make_category_keyboard()).await?;
                }
                _ => {
                    bot.send_message(msg.chat.id,
                                     "Что-то пошло не так... Возможно вы нажали не на ту кнопку").await?;
                }
            }
        }
    }

    bot.answer_callback_query(callback.id).await?;

    Ok(())
}

pub async fn change_model(bot: Bot, msg: Message,
                          dialogue: MyDialogue, state: ChangeModelState) -> HandlerResult {
    match state.type_of_await {
        ChangeModelType::AwaitName => {
            match msg.text() {
                Some(text) => {
                    let mut model = state.model;
                    model.name = text.to_string();
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
                    bot.send_message(msg.chat.id, "Имя должно быть в виде текста!").await?;
                }
            }
        },
        ChangeModelType::AwaitPhoto => {
            match msg.document() {
                Some(document) => {
                    let mut model = state.model;
                    model.media = add_document(document, &bot, MediaType::Image)
                        .await.expect("Can't change document");
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
                    bot.send_message(msg.chat.id, "Не тот тип данных, должно быть фото!")
                        .await?;
                }
            }
        },
        ChangeModelType::AwaitPostName => {
            match msg.text() {
                Some(text) => {
                    dialogue.update(ChangeModel { state: ChangeModelState {
                        model: state.model,
                        type_of_await: ChangeModelType::AwaitPostStatus { name: text.to_string() },
                    }}).await?;
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
        },
        ChangeModelType::AwaitPostMedia {name, status, media_type} => {
            match media_type {
                MediaType::Image => {
                    match msg.document() {
                        Some(document) => {
                            let mut model = state.model;
                            let post = Post {
                                id: Uuid::new_v4(),
                                name,
                                media: add_document(document, &bot, media_type).await.expect("Can't change document"),
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
                            let mut model = state.model;
                            let post = Post {
                                id: Uuid::new_v4(),
                                name,
                                media: add_video(video, &bot, media_type).await.expect("Can't change document"),
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
        },
        _ => {
            bot.send_message(msg.chat.id, "Не подходит. Возможно нужно нажать на кнопку или \
            выйти (/exit)").await?;
        }
    }

    Ok(())
}

fn make_category_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new([
        [
            InlineKeyboardButton::callback("Блогершы", "0"),
            InlineKeyboardButton::callback("Тикток", "1"),
            InlineKeyboardButton::callback("Звезды", "2"),
        ],
        [
            InlineKeyboardButton::callback("Twitch", "3"),
            InlineKeyboardButton::callback("OnlyFans", "4"),
            InlineKeyboardButton::callback("Boosty", "5"),
        ]
    ])
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
        other => {
            let actions = InlineKeyboardMarkup::new([
                vec![
                    InlineKeyboardButton::callback("Удалить", "delete"),
                    InlineKeyboardButton::callback("Перейти к постам", "posts"),
                ],
                vec![
                    InlineKeyboardButton::callback("Изменить имя", "change_name"),
                    InlineKeyboardButton::callback("Изменить фото", "change_photo"),
                ],
                vec![
                    InlineKeyboardButton::callback("Изменить категорию", "change_category"),
                ]]);
            let mut models = db::get_models().await;

            let mut is_found = false;
            for model in models {
                if model.id.to_string() == other {
                    is_found = true;
                    dialogue.update(ChangeModel {
                        state: ChangeModelState {
                            model,
                            type_of_await: ChangeModelType::AwaitCallback
                        },
                    }).await?;
                    bot.send_message(msg.chat.id, "Хорошо, что вы хотите сделать с моделью?")
                        .reply_markup(actions).await?;
                    break;
                }
            }
            if !is_found {
                bot.send_message(msg.chat.id, "Что-то пошло не так, попробуйте еще раз...")
                    .await?;
            }
        }
    }
    bot.answer_callback_query(callback.id).await?;

    Ok(())
}

pub async fn callback_add_new_model(bot: Bot, callback: CallbackQuery, dialogue: MyDialogue,
                                    state: AddNewModelState) -> HandlerResult {
    let msg = callback.message.expect("Message from callback wasn't received");
    let data_option = callback.data.expect("Data from vip callback wasn't received");
    let data = data_option.as_str();
    let mut state = state;

    match state {
        AddNewModelState::ReceiveModelCategory {name} => {
            let category = match data {
                "1" => ModelCategory::Boosty,
                "2" => ModelCategory::Star,
                "3" => ModelCategory::Tiktok,
                "4" => ModelCategory::OnlyFans,
                "5" => ModelCategory::Twitch,
                _ => ModelCategory::Influential
            };
            state = AddNewModelState::ReceiveModelPhoto {name, category};
            dialogue.update(AddNewModel {state}).await?;
            bot.send_message(msg.chat.id, "Отлично, а теперь отправьте фото").await?;
        },
        _ => {
            bot.send_message(msg.chat.id, "Возможно вам нужно было нажать на кнопку!")
                .await?;
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
                    state = AddNewModelState::ReceiveModelCategory {name: text.to_string()};
                    dialogue.update(AddNewModel {state}).await?;
                    let markup = make_category_keyboard();
                    bot.send_message(msg.chat.id, "Отлично, выберите тип модели!")
                        .reply_markup(markup).await?;
                },
                None => {
                    bot.send_message(msg.chat.id, "Нужно ввести имя в виде текста!").await?;
                }
            }
        }
        AddNewModelState::ReceiveModelPhoto {name, category} => {
            match msg.document() {
                Some(document) => {
                    let media = add_document(document, &bot, MediaType::Image).await.unwrap();
                    let mut models = db::get_models().await;
                    models.push(Model {
                        id: Uuid::new_v4(),
                        name,
                        category,
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
        },
        _ => {
            println!("fdsf")
        }
    }

    Ok(())
}

async fn add_document(document: &Document, bot: &Bot, media_type: MediaType) -> Result<Media, ()> {
    let name = document.file_name.clone().unwrap();

    let extension = match Path::new(name.as_str()).extension() {
        None => None,
        Some(path) => path.to_str()
    };

    let folder = "media/";
    let path: &Path = Path::new(folder);
    if !path.exists() {
        create_dir_all(path).await.expect("Can't create dirs");
    }

    let media = Media {
        media_type,
        path: folder.to_string().add(match extension {
            Some(text) => Uuid::new_v4().to_string().add(".").add(text),
            None => Uuid::new_v4().to_string()
        }.as_str())
    };

    let file =  bot.get_file(&document.file.id).await.unwrap();
    let mut dst = File::create(&media.path).await.unwrap();

    // Download the voice message and write it to the file
    bot.download_file(&file.path, &mut dst).await.unwrap();
    Ok((media))
}

async fn add_video(document: &Video, bot: &Bot, media_type: MediaType) -> Result<Media, ()> {
    let name = document.file_name.clone().unwrap();

    let extension = match Path::new(name.as_str()).extension() {
        None => None,
        Some(path) => path.to_str()
    };

    let folder = "media/";
    let path: &Path = Path::new(folder);
    if !path.exists() {
        create_dir_all(path).await.expect("Can't create dirs");
    }

    let media = Media {
        media_type,
        path: folder.to_string().add(match extension {
            Some(text) => Uuid::new_v4().to_string().add(".").add(text),
            None => Uuid::new_v4().to_string()
        }.as_str())
    };

    let file =  bot.get_file(&document.file.id).await.unwrap();
    let mut dst = File::create(&media.path).await.unwrap();

    // Download the voice message and write it to the file
    bot.download_file(&file.path, &mut dst).await.unwrap();
    Ok((media))
}

async fn delete_document(media: Media) {

}