use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

use crate::db::user::User;
use crate::db::{user::UserHandlerTrait, DB};
use crate::error::Result;
use crate::get_user_by_session_token;

use super::gongzuo::{session_token_invalid_error, SessionQuery};

pub async fn users(State(db): State<DB>) -> Result<impl IntoResponse> {
    let users: Vec<_> = db
        .user_handler()
        .users()
        .await?
        .into_iter()
        .filter(|user| !user.is_admin)
        .map(User::from)
        .collect();
    Ok(Json(users))
}

pub async fn me(
    State(db): State<DB>,
    Query(SessionQuery { session_token }): Query<SessionQuery>,
) -> Result<impl IntoResponse> {
    let user = get_user_by_session_token!(db, session_token);

    Ok((StatusCode::OK, Json(json!(User::from(user)))))
}
