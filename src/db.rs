use std::result::Result::Ok;

use anyhow::anyhow;
use futures::TryStreamExt;
use sqlx::Row;

use crate::books::Book;
// TODO: everything should be lowercase
// TODO: everything should be lowercase

pub async fn create_book(book: &Book, pool: &sqlx::PgPool) -> anyhow::Result<()> {
    let query = "INSERT INTO books (title, author, book_path, description, file_size, language, genre) VALUES ($1,$2,$3,$4,$5,$6,$7)";
    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.book_path)
        .bind(&book.description)
        .bind(book.file_size)
        .bind(&book.language)
        .bind(&book.genres)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn search_book_by_name(name: &str, pool: &sqlx::PgPool) -> anyhow::Result<Vec<Book>> {
    let mut books: Vec<Book> = Vec::with_capacity(5);
    name.to_owned().to_lowercase().push('%');
    let q = "SELECT title, author, book_path, description, download_count, file_size, language, genre FROM books WHERE title ILIKE '%' || $1 || '%' ORDER BY similarity(title, $1) DESC LIMIT 5;";
    let mut rows = sqlx::query(q).bind(name).fetch(pool);

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
    let q = "SELECT title, author, book_path, description, download_count, file_size, language, genre FROM books ORDER BY download_count DESC LIMIT 5;";
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

pub async fn get_path(exact_name: &str, pool: &sqlx::PgPool) -> anyhow::Result<String> {
    let q = "SELECT book_path FROM books WHERE title = $1;";
    let query = sqlx::query(q).bind(exact_name);
    let row = query.fetch_one(pool).await?;

    match update_download_count(exact_name, pool).await {
        Ok(_) => Ok(row.get("book_path")),
        Err(err) => {
            log::error!("{:#?}", err);
            Err(anyhow!("{:#?}", err))
        }
    }
}

pub async fn update_download_count(exact_name: &str, pool: &sqlx::PgPool) -> anyhow::Result<()> {
    let query = "UPDATE books SET download_count = download_count + 1 WHERE title = $1;";
    sqlx::query(query).bind(exact_name).execute(pool).await?;
    Ok(())
}
