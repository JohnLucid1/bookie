use crate::{
    books::FileType,
    db::{get_book_path, search_book_by_name},
    HandlerResult, MyDialogue,
};
use std::{fs, path::PathBuf};
use teloxide::{
    net::Download,
    payloads::SendMessageSetters,
    requests::Requester,
    types::{CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup, InputFile, Message},
    Bot,
};
use tokio::fs::File;
// TODO: refactor upload book

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    UploadBook,
    SearchBook,
    ReceiveBookChoice,
}

const BOOKS_DIR_PATH: &str = "./books/";
impl State {
    pub async fn upload_book(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
        match msg.document() {
            Some(document) => {
                if let Some(name) = document.file_name.clone() {
                    let books_path = PathBuf::from(BOOKS_DIR_PATH);
                    // tel file id
                    let file_id = document.file.id.clone(); // OR unique_id
                    let tlx_file = bot.get_file(file_id).await?;
                    // create newpath
                    let file_path = PathBuf::from(name);
                    let combined_path = books_path.join(file_path);
                    let mut dst = File::create(&combined_path).await?;
                    match bot.download_file(&tlx_file.path, &mut dst).await {
                        Ok(_) => match FileType::parse(&combined_path).await {
                            Ok(_) => {
                                bot.send_message(
                                    msg.chat.id,
                                    "Book created :)\nNow you can search for it",
                                )
                                .await?;
                                dialogue.update(State::Start).await?;
                            }
                            Err(err) => {
                                log::error!("ERROR: {}", err);
                                bot.send_message(msg.chat.id, "Some error happened").await?;
                                dialogue.update(State::Start).await?;
                            }
                        },
                        Err(err) => {
                            // Remove file if coundn't downlaod data
                            match fs::remove_file(&combined_path) {
                                Ok(_) => {
                                    log::info!(
                                        "Removed file: {}",
                                        &combined_path.to_str().unwrap()
                                    );
                                    bot.send_message(
                                        msg.chat.id,
                                        format!("Error downlaoding the file: {}", err),
                                    )
                                    .await?;
                                    dialogue.update(State::Start).await?;
                                }
                                Err(err) => {
                                    log::error!("ERROR removeing file: {}", err);
                                    dialogue.update(State::Start).await?;
                                }
                            }
                        }
                    }
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
        // NOTE: msg.chat.id and dialouge.chat_id() are the same

        let db_url = std::env::var("DB_URL").expect("Coudln't get url from .env file");
        let connection = sqlx::postgres::PgPool::connect(&db_url)
            .await
            .expect("Couldn't connect  to db");

        match msg.text().map(ToOwned::to_owned) {
            Some(book_name) => match search_book_by_name(&book_name, &connection).await {
                Ok(books) => {
                    if books.is_empty() {
                        bot.send_message(msg.chat.id, "No books found :(").await?;
                        dialogue.update(State::Start).await?;
                    } else {
                        let books = books.into_iter().map(|book| {
                            InlineKeyboardButton::callback(book.title.clone(), book.title)
                        });

                        bot.send_message(msg.chat.id, "Select a book:")
                            .reply_markup(InlineKeyboardMarkup::new([books]))
                            .await?;
                        dialogue.update(State::ReceiveBookChoice).await?;
                    }
                }
                Err(err) => {
                    log::error!("{:?}", err);
                    bot.send_message(msg.chat.id, "Some error with db").await?;
                    dialogue.update(State::Start).await?;
                }
            },
            None => {
                bot.send_message(msg.chat.id, "Action cancelled").await?;
                dialogue.update(State::Start).await?;
            }
        }

        Ok(())
    }
}

pub async fn receive_book_choice(
    bot: Bot,
    dialogue: MyDialogue,
    q: CallbackQuery,
) -> HandlerResult {
    if let Some(exact_name) = &q.data {
        let db_url = std::env::var("DB_URL").expect("Coudln't get url from .env file");
        let connection = sqlx::postgres::PgPool::connect(&db_url)
            .await
            .expect("Couldn't connect  to db");

        match get_book_path(exact_name, &connection).await {
            Ok(path) => {
                let input = InputFile::file(path);
                bot.send_document(dialogue.chat_id(), input).await?;
                dialogue.exit().await?;
            }
            Err(err) => {
                log::error!("{:#?}", err);
                dialogue.exit().await?;
            }
        }
    }
    Ok(())
}
