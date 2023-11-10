use anyhow::anyhow;
use epub::doc::{DocError, EpubDoc};
use std::{fs::File, io::BufReader, path::Path};
// TODO: implement parse_fb2
// TODO: Delete filesize (cause telegram shows it on download) 
use crate::db::create_book;

#[derive(Debug)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub book_path: String,
    pub description: String,
    pub download_count: i32,
    pub file_size: i32,
    pub language: String,
    pub genres: Vec<String>,
}

pub struct FileType {}

impl FileType {
    pub async fn parse(path: &Path) -> anyhow::Result<()> {
        let db_url = std::env::var("DB_URL").expect("Coudln't get url from .env file");
        let connection = sqlx::postgres::PgPool::connect(&db_url)
            .await
            .expect("Couldn't connect  to db");

        if let Some(os_exstension) = path.extension() {
            let extension = os_exstension.to_str().unwrap_or("");

            match extension {
                "epub" => {
                    let new_book = FileType::parse_epub(path).expect("Couldn't parse epub");
                    create_book(&new_book, &connection).await
                }
                _ => Err(anyhow::anyhow!("ERRORRRR")),
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
        let file_size = File::open(temp_path).unwrap().metadata().unwrap().len() as i32; 
        let language = FileType::parse_tag(&doc, "language").unwrap_or("".into());
        let genres = FileType::parse_tags(&doc, "subject").unwrap_or_default();
        let book_path = temp_path.to_string_lossy().into();

        let new_book = Book {
            title,
            author,
            book_path,
            description,
            download_count: 0,
            file_size,
            language,
            genres,
        };
        Ok(new_book)
    }


}
