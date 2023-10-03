use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Deserialize;
use serde_json::json;

use crate::{db::DB, error::Result};

#[derive(Deserialize, Debug, Clone)]
pub struct LogoutPayload {
    pub session_token: String,
}

pub async fn logout(
    Extension(db): Extension<DB>,
    Json(payload): Json<LogoutPayload>,
) -> Result<impl IntoResponse> {
    let LogoutPayload { session_token } = payload;

    let Some(user) = db.ensure_session_token(&session_token).await? else {
        return Ok((
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "message": "Invalid session token"
            })),
        ));
    };

    db.remove_session_token(user.id).await?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "message": "Logout successful"
        })),
    ))
}
