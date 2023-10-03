pub mod user;

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

    pub async fn users(&self) -> Vec<User> {
        sqlx::query_as!(User, "SELECT * FROM users")
            .fetch_all(&self.pool)
            .await
            .unwrap()
    }
}
