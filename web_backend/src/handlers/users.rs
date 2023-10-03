use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;

use crate::db::user::User;
use crate::db::{user::UserHandlerTrait, DB};
use crate::error::Result;

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
