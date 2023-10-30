use teloxide::{Bot, dptree};
use teloxide::dispatching::UpdateFilterExt;
use teloxide::dptree::entry;
use teloxide::prelude::{Dispatcher, Requester};
use teloxide::types::{Message, Update};

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

struct Person {
    name: String,
    photo: Photo
}

struct Entity {
    person: Person,
    photo: Photo,
    is_premium: bool
}

struct Photo {
    id: u64
}

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceivePerson,
    BranchToChangePersonPhoto,
    BranchToChangePersonName,
    BranchToGetEntitiesByPerson,
    BranchToAddPerson
}

#[tokio::main]
pub async fn start() {
    let bot = Bot::new("6446623321:AAEB5CFL6mcRvLRCJ5ib52fjOkToOtv28WU");

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
    bot.send_message(msg.chat.id, "hello!").await?;
    Ok(())
}