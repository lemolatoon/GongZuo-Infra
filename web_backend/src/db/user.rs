use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Postgres;

#[derive(sqlx::FromRow, Deserialize, Debug)]
pub struct UserRaw {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub salt: String,
    pub created_at: NaiveDateTime,
    #[sqlx(default)]
    pub session_token: Option<String>,
    pub is_admin: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub created_at: DateTime<Utc>,
}

impl From<UserRaw> for User {
    fn from(value: UserRaw) -> Self {
        let UserRaw {
            id,
            username,
            created_at,
            ..
        } = value;

        let created_at: DateTime<Utc> = DateTime::from_naive_utc_and_offset(created_at, Utc);
        User {
            id,
            username,
            created_at,
        }
    }
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
    async fn get_user_by_username(&self, username: &str) -> anyhow::Result<Option<UserRaw>>;
    async fn users(&self) -> anyhow::Result<Vec<UserRaw>>;
    async fn register_user(
        &self,
        username: &str,
        hashed_password: &str,
        salt: &str,
    ) -> anyhow::Result<()>;
    async fn update_session_token(&self, user_id: i32, session_token: &str) -> anyhow::Result<()>;
    async fn ensure_session_token(&self, session_token: &str) -> anyhow::Result<Option<UserRaw>>;
    async fn remove_session_token(&self, user_id: i32) -> anyhow::Result<()>;
    async fn ensure_admin_user_is_registered(&self, username: &str) -> anyhow::Result<bool>;
}

#[axum::async_trait]
impl UserHandlerTrait for UserHandler<'_> {
    async fn get_user_by_username(&self, username: &str) -> anyhow::Result<Option<UserRaw>> {
        let user = sqlx::query_as!(
            UserRaw,
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

    async fn users(&self) -> anyhow::Result<Vec<UserRaw>> {
        let users = sqlx::query_as!(UserRaw, "SELECT * FROM users")
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

    async fn ensure_session_token(&self, session_token: &str) -> anyhow::Result<Option<UserRaw>> {
        let user = sqlx::query_as!(
            UserRaw,
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
        let user = sqlx::query_as!(
            UserRaw,
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
