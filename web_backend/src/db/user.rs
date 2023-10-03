use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::Postgres;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub salt: String,
    pub created_at: NaiveDateTime,
    #[sqlx(default)]
    pub session_token: Option<String>,
    pub is_admin: bool,
}

#[derive(Clone)]
pub struct UserHandler<'a> {
    pool: &'a sqlx::Pool<Postgres>,
}

impl<'a> UserHandler<'a> {
    pub fn new(pool: &'a sqlx::Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[axum::async_trait]
pub trait UserHandlerTrait {
    async fn get_user_by_username(&self, username: &str) -> anyhow::Result<Option<User>>;
    async fn users(&self) -> anyhow::Result<Vec<User>>;
    async fn register_user(
        &self,
        username: &str,
        hashed_password: &str,
        salt: &str,
    ) -> anyhow::Result<()>;
    async fn update_session_token(&self, user_id: i32, session_token: &str) -> anyhow::Result<()>;
    async fn ensure_session_token(&self, session_token: &str) -> anyhow::Result<Option<User>>;
    async fn remove_session_token(&self, user_id: i32) -> anyhow::Result<()>;
    async fn ensure_admin_user_is_registered(&self, username: &str) -> anyhow::Result<bool>;
}

#[axum::async_trait]
impl UserHandlerTrait for UserHandler<'_> {
    async fn get_user_by_username(&self, username: &str) -> anyhow::Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users
            WHERE username = $1
            "#,
            username
        )
        .fetch_optional(self.pool)
        .await?;

        Ok(user)
    }

    async fn users(&self) -> anyhow::Result<Vec<User>> {
        let users = sqlx::query_as!(User, "SELECT * FROM users")
            .fetch_all(self.pool)
            .await?;

        Ok(users)
    }

    async fn register_user(
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
        .execute(self.pool)
        .await?;

        Ok(())
    }

    async fn update_session_token(&self, user_id: i32, session_token: &str) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            UPDATE users
            SET session_token = $1
            WHERE id = $2
            "#,
            session_token,
            user_id
        )
        .execute(self.pool)
        .await?;

        Ok(())
    }

    async fn remove_session_token(&self, user_id: i32) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            UPDATE users
            SET session_token = NULL
            WHERE id = $1
            "#,
            user_id
        )
        .execute(self.pool)
        .await?;

        Ok(())
    }

    async fn ensure_session_token(&self, session_token: &str) -> anyhow::Result<Option<User>> {
        let user: Option<User> = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users
            WHERE session_token = $1
            "#,
            session_token
        )
        .fetch_optional(self.pool)
        .await?;

        Ok(user)
    }

    async fn ensure_admin_user_is_registered(&self, username: &str) -> anyhow::Result<bool> {
        let user: Option<User> = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users
            WHERE username = $1 AND is_admin = true
            "#,
            username
        )
        .fetch_optional(self.pool)
        .await?;

        Ok(user.is_some())
    }
}
