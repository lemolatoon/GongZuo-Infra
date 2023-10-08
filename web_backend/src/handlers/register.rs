use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::handlers::gongzuo::session_token_invalid_error;
use crate::{
    db::{
        user::{User, UserHandlerTrait},
        DB,
    },
    error::AppError,
    get_user_by_session_token,
};

use super::gongzuo::SessionQuery;

#[derive(Deserialize, Debug, Clone)]
pub struct UserPayload {
    pub username: String,
    pub password: String,
}

pub async fn register(
    State(db): State<DB>,
    Query(SessionQuery { session_token }): Query<SessionQuery>,
    Json(payload): Json<UserPayload>,
) -> Result<impl IntoResponse, AppError> {
    let user = get_user_by_session_token!(db, session_token);
    if !user.is_admin {
        return Ok((
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "message": "Only admin can register new users"
            })),
        ));
    }
    let UserPayload { username, password } = payload;

    let user = db.user_handler().get_user_by_username(&username).await?;

    if user.is_some() {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "message": "User already exists"
            })),
        ));
    }

    let (salt, hashed_password) = crate::password::derive(password)?;

    let user = db
        .user_handler()
        .register_user(&username, &hashed_password, &salt)
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(json!({ "user": User::from(user) })),
    ))
}
