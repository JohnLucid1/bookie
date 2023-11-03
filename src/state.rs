use teloxide::{requests::Requester, types::Message, Bot};

use crate::{HandlerResult, MyDialogue};

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveBook,
    UploadBook,
    SearchBook,
}

impl State {
    pub async fn upload_book(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
        match msg.document() {
            Some(document) => {
                //NOTE: this is a test and just suppose to give back the name of the book
                if let Some(name) = document.file_name.clone() {
                    bot.send_message(msg.chat.id, format!("name: {}", name))
                        .await?;
                    dialogue.update(State::Start).await?;
                }
            }
            None => {
                bot.send_message(msg.chat.id, "No documents were found :(")
                    .await?;
                dialogue.update(State::Start).await?;
            }
        }
        Ok(())
    }

    pub async fn searching_book(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
        match msg.text().map(ToOwned::to_owned) {
            Some(book_name) => {
                bot.send_message(msg.chat.id, format!("Searched book: {}", book_name))
                    .await?;
                dialogue.update(State::Start).await?;
            }
            None => {
                bot.send_message(msg.chat.id, "").await?;
                dialogue.update(State::Start).await?;
            }
        }

        Ok(())
    }

    pub async fn receive_book(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
        match msg.document() {
            Some(doc) => {
                let document_name = doc.file_name.clone().unwrap_or("None".into());
                bot.send_message(msg.chat.id, format!("Book name: {}", document_name))
                    .await?;
                dialogue.exit().await?
            }
            None => {
                bot.send_message(msg.chat.id, "Something went wrong")
                    .await?;
            }
        }
        Ok(())
    }
}
