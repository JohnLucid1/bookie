use teloxide::{
    requests::Requester,
    types::Message,
    utils::command::{self, BotCommands},
    Bot,
};

use crate::{state::State, HandlerResult, MyDialogue};

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
}

impl Command {
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
