mod books;
mod commands;
mod db;
mod state;
mod tests;

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

/* NOTE:
    genre can be null
    if something essensial like author or else is null check for similar words
*/

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenv().ok();
    log::info!("Starting bot :)");
    let bot_token = std::env::var("TELOXIDE_TOKEN").expect("Couldn't get token from .env file");
    // let db_url = std::env::var("DB_URL").expect("Coudln't get url from .env file");
    // let pool = sqlx::postgres::PgPool::connect(&db_url)
    //     .await
    //     .expect("Couldn't connect to db");

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
            case![State::Start] // This is setting the state
                .branch(case![Command::Help].endpoint(Command::help))
                .branch(case![Command::SearchBook].endpoint(Command::search_book))
                .branch(case![Command::UploadBook].endpoint(Command::upload_book)),
        )
        .branch(case![Command::Cancel].endpoint(Command::cancel));

    let message_handler = Update::filter_message()
        .branch(command_handler) // This acts upon the state
        .branch(case![State::SearchBook].endpoint(State::searching_book))
        .branch(case![State::ReceiveBook].endpoint(State::receive_book))
        .branch(case![State::UploadBook].endpoint(State::upload_book))
        .branch(dptree::endpoint(invalid_state));

    dialogue::enter::<Update, InMemStorage<State>, State, _>().branch(message_handler)
}

async fn invalid_state(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(
        msg.chat.id,
        "Unable to handle the message. Type /help to see the usage.",
    )
    .await?;
    Ok(())
}

//TODO Make something with this
// async fn receive_product_selection(
//     bot: Bot,
//     dialogue: MyDialogue,
//     full_name: String, // Available from `State::ReceiveProductChoice`.
//     q: CallbackQuery,
// ) -> HandlerResult {
//     if let Some(product) = &q.data {
//         bot.send_message(
//             dialogue.chat_id(),
//             format!("{full_name}, product '{product}' has been purchased successfully!"),
//         )
//         .await?;
//         dialogue.exit().await?;
//     }

//     Ok(())
// }
