use std::result::Result::Ok;

use anyhow::anyhow;
use futures::TryStreamExt;
use sqlx::Row;

use crate::books::Book;
pub struct DB;
impl DB {
    pub async fn create_book(book: &Book, chat_id: i64) -> anyhow::Result<()> {
        let db_url = std::env::var("DB_URL").expect("Coudln't get url from .env file");
        let pool = sqlx::postgres::PgPool::connect(&db_url)
            .await
            .expect("Couldn't connect  to db");

        let insert_query =
            "INSERT INTO books (title, author, book_path, description, language, genre, chat_id) VALUES ($1,$2,$3,$4,$5,$6,$7)";

        sqlx::query(insert_query)
            .bind(&book.title)
            .bind(&book.author)
            .bind(&book.book_path)
            .bind(&book.description)
            .bind(&book.language)
            .bind(&book.genres)
            .bind(chat_id)
            .execute(&pool)
            .await?;

        DB::increment_downloads(&pool, chat_id)
            .await
            .expect("ERROR: cannot increment downlaods :(");
        Ok(())
    }

    pub async fn increment_downloads(pool: &sqlx::PgPool, chat_id: i64) -> anyhow::Result<()> {
        let update_query = "UPDATE users SET books_created = books_created + 1 WHERE chat_id = $1";

        sqlx::query(update_query)
            .bind(chat_id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn search_book_by_name(name: &str) -> anyhow::Result<Vec<Book>> {
        let db_url = std::env::var("DB_URL").expect("Coudln't get url from .env file");
        let pool = sqlx::postgres::PgPool::connect(&db_url)
            .await
            .expect("Couldn't connect  to db");
        let mut books: Vec<Book> = Vec::with_capacity(5);
        name.to_owned().to_lowercase().push('%');
        let q = "SELECT title, author, book_path, description, download_count,chat_id, language, genre FROM books WHERE title ILIKE '%' || $1 || '%' ORDER BY similarity(title, $1) DESC LIMIT 5;";
        let mut rows = sqlx::query(q).bind(name).fetch(&pool);

        while let Some(row) = rows.try_next().await? {
            match Book::row_book(row).await {
                Ok(book) => books.push(book),
                Err(err) => {
                    log::error!("{:#?}", err)
                }
            }
        }

        Ok(books)
    }

    pub async fn get_top_five(pool: &sqlx::PgPool) -> anyhow::Result<Vec<Book>> {
        let mut books = Vec::with_capacity(5);
        let q = "SELECT title, author, book_path, description, download_count,chat_id, language, genre FROM books ORDER BY download_count DESC LIMIT 5;";
        let mut rows = sqlx::query(q).fetch(pool);
        while let Some(row) = rows.try_next().await? {
            match Book::row_book(row).await {
                Ok(book) => books.push(book),
                Err(err) => {
                    log::error!("{:#?}", err)
                }
            }
        }

        Ok(books)
    }

    pub async fn get_users_books(chat_id: i64) -> anyhow::Result<Vec<Book>> {
        let db_url = std::env::var("DB_URL").expect("Coudln't get url from .env file");
        let pool = sqlx::postgres::PgPool::connect(&db_url)
            .await
            .expect("Couldn't connect  to db");

        let mut books: Vec<Book> = Vec::new();
        let q = "SELECT title, author, book_path, description,download_count, language, genre, chat_id FROM books WHERE chat_id = $1";
        let mut rows = sqlx::query(q).bind(chat_id).fetch(&pool);
        while let Some(row) = rows.try_next().await? {
            match Book::row_book(row).await {
                Ok(book) => books.push(book),
                Err(err) => {
                    log::error!("{:?}", err)
                }
            }
        }
        Ok(books)
    }

    pub async fn delete_book(path: &String, chat_id: i64) -> anyhow::Result<()> {
        let db_url = std::env::var("DB_URL").expect("Coudln't get url from .env file");
        let pool = sqlx::postgres::PgPool::connect(&db_url)
            .await
            .expect("Couldn't connect  to db");

        let q = "DELETE FROM books WHERE chat_id = $1 AND book_path = $2";
        sqlx::query(q)
            .bind(chat_id)
            .bind(path)
            .execute(&pool)
            .await?;
        Ok(())
    }

    pub async fn get_book_path(exact_name: &str) -> anyhow::Result<String> {
        let db_url = std::env::var("DB_URL").expect("Coudln't get url from .env file");
        let connection = sqlx::postgres::PgPool::connect(&db_url)
            .await
            .expect("Couldn't connect  to db");

        let q = "SELECT book_path FROM books WHERE title = $1;";
        let query = sqlx::query(q).bind(exact_name);
        let row = query.fetch_one(&connection).await?;

        match DB::update_download_count(exact_name, &connection).await {
            Ok(_) => Ok(row.get("book_path")),
            Err(err) => {
                log::error!("{:#?}", err);
                Err(anyhow!("{:#?}", err))
            }
        }
    }

    pub async fn get_amount_books(chat_id: i64) -> anyhow::Result<i32> {
        let db_url = std::env::var("DB_URL").expect("Coudln't get url from .env file");
        let connection = sqlx::postgres::PgPool::connect(&db_url)
            .await
            .expect("Couldn't connect  to db");

        let q = "SELECT books_created FROM users WHERE chat_id = $1";
        let row = sqlx::query(q).bind(chat_id).fetch_one(&connection).await?;
        let id = row.try_get("books_created").unwrap();
        Ok(id)
    }

    pub async fn update_download_count(
        exact_name: &str,
        pool: &sqlx::PgPool,
    ) -> anyhow::Result<()> {
        let query = "UPDATE books SET download_count = download_count + 1 WHERE title = $1;";
        sqlx::query(query).bind(exact_name).execute(pool).await?;
        Ok(())
    }
}
