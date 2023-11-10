use teloxide::{
    payloads::SendMessageSetters,
    requests::Requester,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, Message},
    utils::command::BotCommands,
    Bot,
};

use crate::{db::get_top_five, state::State, HandlerResult, MyDialogue};

/// These commands are supported:
#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
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
    pub async fn get_top_five(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
        let db_url = std::env::var("DB_URL").expect("Coudln't get url from .env file");
        let connection = sqlx::postgres::PgPool::connect(&db_url)
            .await
            .expect("Couldn't connect  to db");

        let top_books = get_top_five(&connection)
            .await?
            .into_iter()
            .map(|book| InlineKeyboardButton::callback(book.title.clone(), book.title));

        bot.send_message(msg.chat.id, "Select a book:")
            .reply_markup(InlineKeyboardMarkup::new([top_books]))
            .await?;

        dialogue.update(State::ReceiveBookChoice).await?;
        Ok(())
    }

    pub async fn upload_book(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
        bot.send_message(msg.chat.id, "Upload a book").await?;
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
