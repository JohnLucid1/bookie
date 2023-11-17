use anyhow::anyhow;
use epub::doc::{DocError, EpubDoc};
use sqlx::{postgres::PgRow, Row};
use std::{fs::File, io::BufReader, path::Path};
// TODO: implement parse_fb2
// TODO: Refactor all this sheet

use crate::dbs::db::DB;
#[derive(Debug)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub book_path: String,
    pub description: String,
    pub download_count: i32,
    pub language: String,
    pub genres: Vec<String>,
}

impl Book {
    pub async fn row_book(row: PgRow) -> anyhow::Result<Book> {
        let book = Book {
            title: row.try_get("title")?,
            author: row.try_get("author")?,
            book_path: row.try_get("book_path")?,
            description: row.try_get("description")?,
            download_count: row.try_get("download_count")?,
            language: row.try_get("language")?,
            genres: row.try_get("genre")?,
        };

        anyhow::Ok(book)
    }
}

pub struct FileType {}

impl FileType {
    pub async fn parse(path: &Path, chat_id: i64) -> anyhow::Result<()> {
        if let Some(os_exstension) = path.extension() {
            let extension = os_exstension.to_str().unwrap_or("");
            match extension {
                // TODO: change to include the user_id
                "epub" => {
                    let new_book = FileType::parse_epub(path).expect("Couldn't parse epub");
                    DB::create_book(&new_book, chat_id).await
                }
                _ => Err(anyhow!("ERRORRRR")),
            }
        } else {
            Err(anyhow!("Couln't get file extension"))
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
        let language = FileType::parse_tag(&doc, "language").unwrap_or("".into());
        let genres = FileType::parse_tags(&doc, "subject").unwrap_or_default();
        let book_path = temp_path.to_string_lossy().into();

        let new_book = Book {
            title,
            author,
            book_path,
            description,
            download_count: 0,
            language,
            genres,
        };
        Ok(new_book)
    }
}
