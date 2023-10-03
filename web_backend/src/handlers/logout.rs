use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use serde_json::json;

use crate::{
    db::{user::UserHandlerTrait, DB},
    error::Result,
};

#[derive(Deserialize, Debug, Clone)]
pub struct LogoutPayload {
    pub session_token: String,
}

pub async fn logout(
    State(db): State<DB>,
    Json(payload): Json<LogoutPayload>,
) -> Result<impl IntoResponse> {
    let LogoutPayload { session_token } = payload;

    let Some(user) = db
        .user_handler()
        .ensure_session_token(&session_token)
        .await?
    else {
        return Ok((
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "message": "Invalid session token"
            })),
        ));
    };

    db.user_handler().remove_session_token(user.id).await?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "message": "Logout successful"
        })),
    ))
}
