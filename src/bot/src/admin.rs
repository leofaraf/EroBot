use teloxide::{Bot, dptree};
use teloxide::dispatching::UpdateFilterExt;
use teloxide::dptree::entry;
use teloxide::prelude::{Dispatcher, Requester};
use teloxide::types::{Message, Update};
use uuid::Uuid;
use crate::db::{flush_models, get_models, get_models_as_str, Model};

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
pub async fn run() {
    pretty_env_logger::init();
    let bot = Bot::new(
        "6446623321:AAEB5CFL6mcRvLRCJ5ib52fjOkToOtv28WU");

    let message_handler = Update::filter_message()
        .branch(entry().endpoint(gg));

    let handler = entry()
        .branch(message_handler);

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn gg(bot: Bot, msg: Message) -> HandlerResult {
    let mut models = Vec::new();
    models.push(Model {
        id: Uuid::new_v4(),
        name: "ffr".to_string(),
        photo_path: "".to_string(),
        posts: vec![],
    });
    flush_models(models).await;

    let text = get_models_as_str().await;


    bot.send_message(msg.chat.id, text).await?;

    Ok(())
}