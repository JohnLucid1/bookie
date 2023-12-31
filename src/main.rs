mod books;
mod commands;
mod dbs;
mod state;
mod tests;
mod users;

use commands::Command;
use dotenv::dotenv;
use state::State;
use teloxide::{
    dispatching::{
        dialogue::{self, InMemStorage},
        UpdateHandler,
    },
    prelude::*,
};
type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenv().ok();
    log::info!("Starting bot :)");
    let bot_token = std::env::var("TELOXIDE_TOKEN").expect("Couldn't get token from .env file");
    let bot = Bot::new(bot_token);
    Dispatcher::builder(bot, schema())
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(
            case![State::Start] //NOTE:  This is setting the state
                .branch(case![Command::Start].endpoint(Command::start))
                .branch(case![Command::Help].endpoint(Command::help))
                .branch(case![Command::SearchBook].endpoint(Command::search_book))
                .branch(case![Command::UploadBook].endpoint(Command::upload_book))
                .branch(case![Command::TopFive].endpoint(Command::get_top_five))
                .branch(case![Command::DeleteBook].endpoint(Command::delete_book))
                .branch(case![Command::MyBooks].endpoint(Command::my_books)),
        )
        .branch(case![Command::Cancel].endpoint(Command::cancel));

    let message_handler = Update::filter_message()
        .branch(command_handler) //NOTE: This acts upon the state
        .branch(case![State::SearchBook].endpoint(State::searching_book))
        .branch(case![State::UploadBook].endpoint(State::upload_book))
        .branch(dptree::endpoint(invalid_state));

    let callback_query_handler = Update::filter_callback_query()
        .branch(case![State::ReceiveBookChoice].endpoint(state::receive_book_choice))
        .branch(case![State::ReceiveBookDelete].endpoint(state::receive_book_delete));

    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(message_handler)
        .branch(callback_query_handler)
}

async fn invalid_state(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    bot.send_message(
        msg.chat.id,
        "Unable to handle the message. Type /help to see the usage.",
    )
    .await?;
    dialogue.exit().await?;
    Ok(())
}
