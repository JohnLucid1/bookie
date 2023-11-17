use crate::users::User;

use anyhow::anyhow;

pub struct Usr;

impl Usr {
    pub async fn create_new_user(pool: &sqlx::PgPool, new_user: &User) -> anyhow::Result<()> {
        let q = "SELECT user_id, books_created,chat_id, is_admin FROM users WHERE chat_id = $1";
        let rows = sqlx::query(q)
            .bind(new_user.chat_id)
            .fetch_optional(pool)
            .await?;
        match rows.is_some() {
            true => Ok(()),
            false => match Usr::create_user(new_user, pool).await {
                Ok(()) => Ok(()),
                Err(err) => {
                    log::error!("{:?}", err);
                    Err(anyhow!("Couldn't create new user: {}", err))
                }
            },
        }
    }

    async fn create_user(new_user: &User, pool: &sqlx::PgPool) -> anyhow::Result<()> {
        let query = "INSERT INTO users (chat_id) VALUES ($1)";
        sqlx::query(query)
            .bind(new_user.chat_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
