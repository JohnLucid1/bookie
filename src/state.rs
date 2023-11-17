use crate::{books::FileType, dbs::db::DB, HandlerResult, MyDialogue};
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
// TODO: Impliment deleting books

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    UploadBook,
    SearchBook,
    ReceiveBookChoice,
    // DeleteBook,
    ReceiveBookDelete,
}

const BOOKS_DIR_PATH: &str = "./books/";
impl State {
    pub async fn upload_book(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
        let amount = DB::get_amount_books(msg.chat.id.0).await.unwrap();
        if amount < 5 {
            match msg.document() {
                Some(document) => {
                    let name = document.file_name.clone(); // TODO: refactor this bullshit
                    if name.is_none() {
                        log::error!("Document has no filename :(");
                        bot.send_message(msg.chat.id, "Cannot get the name of the book :(")
                            .await?;
                        dialogue.update(State::Start).await?;
                    }
                    let name = name.unwrap();
                    let books_path = PathBuf::from(BOOKS_DIR_PATH);
                    // tel file id
                    let file_id = document.file.id.clone(); // OR unique_id
                    let tlx_file = bot.get_file(file_id).await?;
                    // create newpath
                    let file_path = PathBuf::from(name);
                    let combined_path = books_path.join(file_path);
                    let mut dst = File::create(&combined_path).await?;
                    match bot.download_file(&tlx_file.path, &mut dst).await {
                        Ok(_) => match FileType::parse(&combined_path, msg.chat.id.0).await {
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
                        Err(_) => {
                            // Remove file if coundn't downlaod data
                            match fs::remove_file(&combined_path) {
                                Ok(_) => {
                                    log::info!(
                                        "Removed file: {}",
                                        &combined_path.to_str().unwrap()
                                    );
                                    bot.send_message(msg.chat.id, "Error downlaoding the file :(")
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
                    //
                }
                None => {
                    bot.send_message(msg.chat.id, "No documents were found :(")
                        .await?;
                    dialogue.update(State::Start).await?;
                }
            }
        } else {
            bot.send_message(
                msg.chat.id,
                "You can't upload more than 5 books.\nDelete some",
            )
            .await?;
            dialogue.update(State::Start).await?;
        }
        Ok(())
    }

    pub async fn searching_book(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
        match msg.text().map(ToOwned::to_owned) {
            Some(book_name) => match DB::search_book_by_name(&book_name).await {
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
        match DB::get_book_path(exact_name).await {
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

pub async fn receive_book_delete(
    bot: Bot,
    dialogue: MyDialogue,
    q: CallbackQuery,
) -> HandlerResult {
    if let Some(book_path) = &q.data {
        let chat_id = dialogue.chat_id().0;
        match DB::delete_book(book_path, chat_id).await {
            Ok(()) => match fs::remove_file(book_path) {
                Ok(res) => {
                    log::info!("Successfully remove file\n {:?}, {}", res, &book_path);
                    bot.send_message(dialogue.chat_id(), "Successfully removed your book")
                        .await?;
                    dialogue.exit().await?;
                }
                Err(err) => {
                    log::error!("Error remove book: {},\nERROR:{:?}", book_path, err);
                    bot.send_message(dialogue.chat_id(), "Error removing file :(")
                        .await?;
                    dialogue.exit().await?;
                }
            },
            Err(err) => {
                log::error!("ERROR deleting book from db: {:#?}", err);
                bot.send_message(dialogue.chat_id(), "ERROR deleting book from db :(")
                    .await?;
                dialogue.update(State::Start).await?;
            }
        }
    }
    Ok(())
}
