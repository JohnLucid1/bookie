use teloxide::{
    payloads::SendMessageSetters,
    requests::Requester,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, Message},
    utils::command::BotCommands,
    Bot,
};

use crate::dbs::{db::DB, users::Usr};
use crate::{state::State, users::User, HandlerResult, MyDialogue};

/// These commands are supported:
#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "Start the process.")]
    Start,
    #[command(description = "Display this text.")]
    Help,
    #[command(description = "Cancel any operation.")]
    Cancel,
    #[command(description = "Search for a book by name.")]
    SearchBook,
    #[command(description = "Uploading a book is for admins only.")]
    UploadBook,
    #[command(description = "Get top 5 downloaded books.")]
    TopFive, // NOTE: no state attached
}

impl Command {
    #[allow(dead_code)]
    pub async fn get_top_five(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
        let db_url = std::env::var("DB_URL").expect("Coudln't get url from .env file");
        let connection = sqlx::postgres::PgPool::connect(&db_url).await?;

        match DB::get_top_five(&connection).await {
            Ok(books) => {
                if books.is_empty() {
                    bot.send_message(msg.chat.id, "No books were found.")
                        .await?;
                    dialogue.update(State::Start).await?;
                } else {
                    let books = books
                        .into_iter()
                        .map(|book| InlineKeyboardButton::callback(book.title.clone(), book.title));
                    bot.send_message(msg.chat.id, "Select a book:")
                        .reply_markup(InlineKeyboardMarkup::new([books]))
                        .await?;
                    dialogue.update(State::ReceiveBookChoice).await?;
                }
            }
            Err(err) => {
                log::error!("{:#?}", err);
                bot.send_message(msg.chat.id, "Server error :(.").await?;
                dialogue.update(State::Start).await?;
            }
        }
        Ok(())
    }

    // NOTE: this is a test of flow (command::start -> state::start) and will it work
    pub async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
        let db_url = std::env::var("DB_URL").expect("Coudln't get url from .env file");
        let connection = sqlx::postgres::PgPool::connect(&db_url)
            .await
            .expect("Couldn't connect  to db");

        let new_user = User::new(msg.chat.id.0);
        match Usr::create_new_user(&connection, &new_user).await {
            Ok(()) => {
                bot.send_message(
                    msg.chat.id,
                    format!(
                        "Your account has been created!!!\n{}",
                        Command::descriptions()
                    ),
                )
                .await?;
                dialogue.update(State::Start).await?;
            }
            Err(err) => {
                log::error!("{:?}", err);
                bot.send_message(
                    msg.chat.id,
                    "Something went wrong :(\nCancelling the dialogue",
                )
                .await?;
                dialogue.reset().await?;
            }
        }
        Ok(())
    }

    pub async fn upload_book(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
        bot.send_message(
            msg.chat.id,
            "Upload a book.\nUnfortunately only .epub format is supported right now.",
        )
        .await?;
        dialogue.update(State::UploadBook).await?;
        Ok(())
    }

    pub async fn search_book(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
        bot.send_message(msg.chat.id, "Enter a book's name").await?;
        dialogue.update(State::SearchBook).await?;
        Ok(())
    }

    pub async fn help(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
        bot.send_message(msg.chat.id, Command::descriptions().to_string())
            .await?;
        dialogue.update(State::Start).await?;
        Ok(())
    }

    pub async fn cancel(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
        bot.send_message(msg.chat.id, "Cancelling the dialogue.")
            .await?;
        dialogue.exit().await?;
        Ok(())
    }
}
