use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use serde_json::json;

use crate::{db::DB, error::AppError};

#[derive(Deserialize, Debug, Clone)]
pub struct UserPayload {
    pub username: String,
    pub password: String,
}

pub async fn register(
    State(db): State<DB>,
    Json(payload): Json<UserPayload>,
) -> Result<impl IntoResponse, AppError> {
    let UserPayload { username, password } = payload;

    let user = db.get_user_by_username(&username).await?;

    if user.is_some() {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "message": "User already exists"
            })),
        ));
    }

    let (salt, hashed_password) = crate::password::derive(password)?;

    db.register_user(&username, &hashed_password, &salt).await?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "message": "User created"
        })),
    ))
}
