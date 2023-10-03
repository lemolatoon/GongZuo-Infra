use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;
use serde_json::json;

use crate::db::gongzuo::{Gongzuo, GongzuoHandlerTrait};
use crate::db::user::UserHandlerTrait;
use crate::db::DB;
use crate::error::Result;
use serde_with::NoneAsEmptyString;

#[serde_with::serde_as]
#[derive(Deserialize, Debug, Clone)]
#[serde(default)]
pub struct SessionQuery {
    #[serde_as(as = "NoneAsEmptyString")]
    pub session_token: Option<String>,
}

#[allow(clippy::derivable_impls)]
impl Default for SessionQuery {
    fn default() -> Self {
        Self {
            session_token: None,
        }
    }
}

fn session_token_invalid_error() -> Result<(StatusCode, axum::Json<serde_json::Value>)> {
    Ok((
        StatusCode::UNAUTHORIZED,
        Json(json!({
            "message": "Invalid session token"
        })),
    ))
}

pub async fn gongzuos(
    State(db): State<DB>,
    Query(SessionQuery { session_token }): Query<SessionQuery>,
) -> Result<impl IntoResponse> {
    let Some(session_token) = session_token else {
        return session_token_invalid_error();
    };

    let Some(user) = db
        .user_handler()
        .ensure_session_token(&session_token)
        .await?
    else {
        return session_token_invalid_error();
    };

    let gongzuos = db.gongzuo_handler().gongzuos_by_user_id(user.id).await?;
    let gongzuos = gongzuos.into_iter().map(Gongzuo::from).collect::<Vec<_>>();
    Ok((StatusCode::OK, Json(json!(gongzuos))))
}
