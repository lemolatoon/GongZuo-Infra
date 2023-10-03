use axum::http::StatusCode;
use axum::{response::IntoResponse, Extension, Json};
use serde::Deserialize;
use serde_json::json;

use crate::db::user::User;
use crate::db::DB;
use crate::error::Result;
use crate::password;
use crate::session::create_session_token;

#[derive(Deserialize, Debug, Clone)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

pub async fn login(
    Extension(db): Extension<DB>,
    Json(payload): Json<LoginPayload>,
) -> Result<impl IntoResponse> {
    let LoginPayload { username, password } = payload;

    let Some(user) = db.get_user_by_username(&username).await? else {
        return Err(anyhow::anyhow!("User {} not found", &username).into());
    };

    let User {
        id: user_id,
        password: hashed_password,
        salt,
        session_token,
        ..
    } = user;

    let is_valid = password::verify_with_salt(salt, hashed_password, password)?;

    if is_valid {
        let session_token = match session_token {
            Some(session_token) => session_token,
            None => {
                let session_token = create_session_token();
                db.update_session_token(user_id, &session_token).await?;

                session_token
            }
        };

        return Ok((
            StatusCode::OK,
            Json(json!({
                "message": "Login successful",
                "session_token": session_token,
            })),
        ));
    } else {
        return Ok((
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "message": "Login failed"
            })),
        ));
    }
}
