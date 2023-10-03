use axum::http::StatusCode;
use axum::{response::IntoResponse, Extension, Json};
use serde::Deserialize;
use serde_json::json;

use crate::db::user::User;
use crate::db::DB;
use crate::error::Result;
use crate::password;

#[derive(Deserialize, Debug, Clone)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

pub async fn login(
    Extension(db): Extension<DB>,
    Json(payload): Json<LoginPayload>,
) -> Result<impl IntoResponse> {
    println!("Login payload: {:?}", &payload);
    let LoginPayload { username, password } = payload;

    let Some(user) = db.get_user_by_username(&username).await? else {
        return Err(anyhow::anyhow!("User {} not found", &username).into());
    };

    println!("User: {:?}", &user);

    let User {
        password: hashed_password,
        salt,
        ..
    } = user;

    let is_valid = password::verify_with_salt(salt, hashed_password, password)?;

    if is_valid {
        return Ok((
            StatusCode::OK,
            Json(json!({
                "message": "Login successful"
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
