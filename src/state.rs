use crate::{books::FileType, db::search_book_by_name, HandlerResult, MyDialogue};
use std::{fs, path::PathBuf};
use teloxide::{net::Download, requests::Requester, types::{Message, InputFile}, Bot};
use tokio::fs::File;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    UploadBook,
    SearchBook,
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
        let db_url = std::env::var("DB_URL").expect("Coudln't get url from .env file");
        let connection = sqlx::postgres::PgPool::connect(&db_url)
            .await
            .expect("Couldn't connect  to db");

        match msg.text().map(ToOwned::to_owned) {
            Some(book_name) => {
                // NOTE: if let cannot capture the error
                match search_book_by_name(&book_name, &connection).await {
                    Ok(books) => {
                        // TODO: create input ifle
                        // let input_file = InputFile::file(path) 
                        println!("{:#?}", books);
                        bot.send_message(msg.chat.id, format!("Searched book: {}", book_name))
                            .await?;
                        dialogue.update(State::Start).await?;
                    }
                    Err(err) => {
                        log::error!("{:?}", err);
                        bot.send_message(msg.chat.id, "Some error with db").await?;
                        dialogue.update(State::Start).await?;
                    }
                }
            }

            None => {
                bot.send_message(msg.chat.id, "Action cancelled").await?;
                dialogue.update(State::Start).await?;
            }
        }

        Ok(())
    }
}
