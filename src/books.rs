use std::{fs::File, io::BufReader, path::Path, error::Error};
#[allow(dead_code)]

use epub::doc::{DocError, EpubDoc};
use sqlx::{Pool, Postgres};

use crate::db::create_book;

pub struct Book {
    pub title: String,
    pub author: String,
    pub book_path: String,
    pub description: String,
    pub download_count: u32,
    pub file_size: u64,
    pub language: String,
    pub genres: Vec<String>,
    pub format: String,
}


struct Config {
    db_url: String, 
    pool: Pool<Postgres>
}

const FOLDER_PATH: &str = "./books";

pub enum FileType {
    EPUB,
    FB2,
}

impl FileType {
    async fn parse(path: &Path) -> Option<Book> {
        let db_url = std::env::var("DB_URL").expect("Coudln't get url from .env file");
        let pool = sqlx::postgres::PgPool::connect(&db_url)
            .await
            .expect("Couldn't connect to db");
        
        if let Some(os_exstension) = path.extension() {
            let extension = os_exstension.to_str().unwrap_or("");
            match extension {
                "epub" => {
                    todo!()
                }, 

                "fb2" => {
                    todo!()
                },

                _ => None,
            }
        } else {
            None
        }
    }

    fn parse_tag(book: &EpubDoc<BufReader<File>>, tag: &str) -> Option<String> {
        book.mdata(tag)
    }

    fn parse_tags(book: &EpubDoc<BufReader<File>>, tag: &str) -> Option<Vec<String>> {
        book.metadata.get(tag).cloned()
    }

    pub fn parse_epub(temp_path: &Path) -> Result<Book, DocError> {
        let doc = EpubDoc::new(temp_path)?;
        let title = FileType::parse_tag(&doc, "title").unwrap(); 
        let author = FileType::parse_tag(&doc, "creator").unwrap_or("".into());
        let description = FileType::parse_tag(&doc, "description").unwrap_or("".into());
        let file_size = File::open(&temp_path).unwrap().metadata().unwrap().len(); // TODO: either this or get from tg message data
        let language = FileType::parse_tag(&doc, "language").unwrap_or("".into());
        let genres = FileType::parse_tags(&doc, "subject").unwrap_or(Vec::new());
        let book_path = temp_path.to_string_lossy().into();
        let format = ".epub".into();

        let new_book = Book {
            title,
            author,
            book_path,
            description,
            download_count: 0,
            file_size,
            language,
            genres,
            format,
        };
        
        Ok(new_book)
    }
    

    fn save_fb2(path: &Path) -> Result<Book, DocError> {
        todo!();
    }
}
