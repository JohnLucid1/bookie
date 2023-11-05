use std::error::Error;

use crate::books::Book;

pub async fn create_book(book: &Book, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO books (title, author, book_path, description, file_size, langauge, genre) VALUES ($1,$2,$3,$4,$5,$6,$7)";

    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.book_path)
        .bind(&book.description)
        .bind(book.file_size as i64)
        .bind(&book.language)
        .bind(&book.genres)
        .execute(pool)
        .await?;

    Ok(())
}