use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

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
