use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::macros::BotCommands as Commands;
use teloxide::prelude::Dialogue;
use crate::db::models::{Media, MediaType, Model, ModelCategory};

pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
pub type MyDialogue = Dialogue<State, InMemStorage<State>>;

#[derive(Commands, Clone)]
#[command(rename_rule = "lowercase", description = "Существуют эти комманды:")]
pub enum Command {
    #[command(description = "получитить основную информацию (вы тут)")]
    Start,
    #[command(description = "меню для изменения моделей или медиа")]
    Models,
    #[command(description = "меню для добавления/удаления VIP пользователей")]
    Vip,
    #[command(description = "(ВАЖНО) если вы хотите выйти диалога, просто напишите эту комманду")]
    Exit
}


#[derive(Clone)]
pub enum AddNewVipState {
    AwaitId,
    AwaitName {id: isize}
}

#[derive(Clone)]
pub enum ChangeModelType {
    AwaitCallback,
    AwaitName,
    AwaitCategory,
    AwaitPhoto,
    AwaitPostAction,
    AwaitPostName,
    AwaitPostStatus {name: String},
    AwaitPostMediaType {name: String, status: bool},
    AwaitPostMedia {name: String, status: bool, media_type: MediaType},
}

#[derive(Clone)]
pub struct ChangeModelState {
    pub model: Model,
    pub type_of_await: ChangeModelType
}


#[derive(Clone)]
pub enum AddNewModelState {
    ReceiveModelName,
    ReceiveModelCategory {name: String},
    ReceiveModelPhoto {name: String, category: ModelCategory}
}

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    Models,
    AddNewModel {state: AddNewModelState},
    RemoveModel,
    ChangeModel {state: ChangeModelState},
    Vip,
    AddNewVip {state: AddNewVipState},
    RemoveVip
}