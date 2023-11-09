use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dispatching::UpdateHandler;
use teloxide::macros::BotCommands as Commands;
use teloxide::prelude::Dialogue;
use crate::db::models::{Media, MediaType, Model, ModelCategory};

pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
pub type MyDialogue = Dialogue<State, InMemStorage<State>>;
pub type Schema = UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>>;

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
    AwaitPhoto
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

    AwaitPostAction {model: Model},
    AwaitPostName {model: Model},
    AwaitPostStatus {model: Model, name: String},
    AwaitPostMediaType {model: Model, name: String, status: bool},
    AwaitPostMedia {model: Model, name: String, status: bool, media_type: MediaType},

    Vip,
    AddNewVip {state: AddNewVipState},
    RemoveVip
}