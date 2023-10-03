pub mod user;

use anyhow::Ok;
use sqlx::{Pool, Postgres};
use user::User;

#[derive(Clone)]
pub struct DB {
    pool: Pool<Postgres>,
}

impl DB {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn get_user_by_username(&self, username: &str) -> anyhow::Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users
            WHERE username = $1
            "#,
            username
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn users(&self) -> anyhow::Result<Vec<User>> {
        let users = sqlx::query_as!(User, "SELECT * FROM users")
            .fetch_all(&self.pool)
            .await?;

        Ok(users)
    }

    pub async fn register_user(
        &self,
        username: &str,
        hashed_password: &str,
        salt: &str,
    ) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO users (username, password, salt)
            VALUES ($1, $2, $3)
            "#,
            username,
            hashed_password,
            salt
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
