use anyhow::Ok;
use futures::TryStreamExt;
use sqlx::Row;

use crate::books::Book;

pub async fn create_book(book: &Book, pool: &sqlx::PgPool) -> anyhow::Result<()> {
    let query = "INSERT INTO books (title, author, book_path, description, file_size, language, genre) VALUES ($1,$2,$3,$4,$5,$6,$7)";
    sqlx::query(query) // TODO: everything should be lowercase
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
    // TODO: everything should be lowercase
    let mut books: Vec<Book> = Vec::with_capacity(5);
    name.to_owned().push('%');
    // let q = "SELECT title, author, book_path, description, download_count, file_size, language, genre FROM books WHERE title SIMILAR TO ? ORDER BY similarity(title, ?) DESC";
    let q = "SELECT title, author, book_path, description, download_count, file_size, language, genre FROM books WHERE title LIKE $1";


    let mut rows = sqlx::query(q).bind(name).fetch(pool);

    while let Some(row) = rows.try_next().await? {
        let book = Book {
            title: row.try_get("title")?,
            author: row.try_get("author")?,
            book_path: row.try_get("book_path")?,
            description: row.try_get("description")?,
            download_count: row.try_get("download_count")?,
            file_size: row.try_get("file_size")?,
            language: row.try_get("language")?,
            genres: row.try_get("genre")?,
        };
        books.push(book)
    }

    Ok(books)
}
