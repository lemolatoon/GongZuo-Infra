use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::json;

use crate::db::gongzuo::{ContentKind, Gongzuo, GongzuoHandlerTrait, GongzuoPayload};
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

pub fn session_token_invalid_error() -> Result<(StatusCode, axum::Json<serde_json::Value>)> {
    Ok((
        StatusCode::UNAUTHORIZED,
        Json(json!({
            "message": "Invalid session token"
        })),
    ))
}

#[macro_export]
macro_rules! get_user_by_session_token {
    ($db:expr, $session_token:expr) => {{
        let Some(session_token) = $session_token else {
            return session_token_invalid_error();
        };

        match $db
            .user_handler()
            .ensure_session_token(&session_token)
            .await?
        {
            Some(user) => user,
            None => return session_token_invalid_error(),
        }
    }};
}

pub async fn all_ongzuos(
    State(db): State<DB>,
    Query(SessionQuery { session_token }): Query<SessionQuery>,
) -> Result<impl IntoResponse> {
    let _user = get_user_by_session_token!(db, session_token);

    let gongzuos = db.gongzuo_handler().all_gongzuos().await?;
    let gongzuos = gongzuos.into_iter().map(Gongzuo::from).collect::<Vec<_>>();
    Ok((StatusCode::OK, Json(json!(gongzuos))))
}

#[derive(Deserialize, Debug, Clone)]
pub struct GongzuoStartPayload {
    pub content_kind: ContentKind,
    pub content: String,
}

pub async fn start_gongzuo(
    State(db): State<DB>,
    Query(SessionQuery { session_token }): Query<SessionQuery>,
    Json(payload): Json<GongzuoStartPayload>,
) -> Result<impl IntoResponse> {
    let user = get_user_by_session_token!(db, session_token);

    let GongzuoStartPayload {
        content_kind,
        content,
    } = payload;

    let started_at = Utc::now();

    let payload = GongzuoPayload {
        started_at,
        ended_at: None,
        content_kind,
        content,
    };

    let gongzuo_id = db
        .gongzuo_handler()
        .create_gongzuo(user.id, payload)
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(json!({ "gongzuo_id": gongzuo_id })),
    ))
}

#[derive(Deserialize, Debug, Clone)]
pub struct GongzuoEndPayload {
    pub gongzuo_id: i32,
}

pub async fn end_gongzuo(
    State(db): State<DB>,
    Query(SessionQuery { session_token }): Query<SessionQuery>,
    Json(payload): Json<GongzuoEndPayload>,
) -> Result<impl IntoResponse> {
    let user = get_user_by_session_token!(db, session_token);

    let GongzuoEndPayload { gongzuo_id } = payload;

    let Some(gongzuo) = db
        .gongzuo_handler()
        .gongzuo_by_gongzuo_id(gongzuo_id)
        .await?
    else {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "message": format!("Gongzuo {} not found", gongzuo_id)
            })),
        ));
    };

    if gongzuo.ended_at.is_some() {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "message": format!("Gongzuo {} already ended", gongzuo_id)
            })),
        ));
    }

    let Gongzuo {
        id: gongzuo_id,
        started_at,
        content_kind,
        content,
        ..
    } = Gongzuo::from(gongzuo);

    let started_at = started_at.with_timezone(&Utc);
    let ended_at = Some(Utc::now());

    let payload = GongzuoPayload {
        started_at,
        ended_at,
        content_kind,
        content,
    };

    if let Err(error_message) = db
        .gongzuo_handler()
        .update_gongzuo(gongzuo_id, user.id, payload)
        .await?
    {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "message": error_message,
            })),
        ));
    }

    Ok((
        StatusCode::OK,
        Json(json!({
            "ended_at": ended_at,
            "message": "Gongzuo ended",
        })),
    ))
}

#[derive(Deserialize, Debug, Clone)]
pub struct GongzuoEditPayload {
    pub gongzuo_id: i32,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub content_kind: ContentKind,
    pub content: String,
}

pub async fn edit_gongzuo(
    State(db): State<DB>,
    Query(SessionQuery { session_token }): Query<SessionQuery>,
    Json(payload): Json<GongzuoEditPayload>,
) -> Result<impl IntoResponse> {
    let user = get_user_by_session_token!(db, session_token);

    let GongzuoEditPayload {
        gongzuo_id,
        started_at,
        ended_at,
        content_kind,
        content,
    } = payload;

    let payload = GongzuoPayload {
        started_at,
        ended_at,
        content_kind,
        content,
    };

    if let Err(error_message) = db
        .gongzuo_handler()
        .update_gongzuo(gongzuo_id, user.id, payload)
        .await?
    {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "message": error_message,
            })),
        ));
    }

    Ok((
        StatusCode::OK,
        Json(json!({
            "message": "Gongzuo updated",
        })),
    ))
}

#[derive(Deserialize, Debug, Clone)]
pub struct GongzuoDeletePayload {
    pub gongzuo_id: i32,
}

pub async fn delete_gongzuo(
    State(db): State<DB>,
    Query(SessionQuery { session_token }): Query<SessionQuery>,
    Json(payload): Json<GongzuoDeletePayload>,
) -> Result<impl IntoResponse> {
    let user = get_user_by_session_token!(db, session_token);

    let GongzuoDeletePayload { gongzuo_id } = payload;

    if let Err(error_message) = db
        .gongzuo_handler()
        .delete_gongzuo(gongzuo_id, user.id)
        .await?
    {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "message": error_message,
            })),
        ));
    }

    Ok((
        StatusCode::OK,
        Json(json!({
            "message": "Gongzuo deleted",
        })),
    ))
}

pub async fn gongzuo_by_id(
    State(db): State<DB>,
    Query(SessionQuery { session_token }): Query<SessionQuery>,
    Path(gongzuo_id): Path<i32>,
) -> Result<impl IntoResponse> {
    let _user = get_user_by_session_token!(db, session_token);

    let Some(gongzuo) = db
        .gongzuo_handler()
        .gongzuo_by_gongzuo_id(gongzuo_id)
        .await?
    else {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "message": format!("Gongzuo {} not found", gongzuo_id)
            })),
        ));
    };

    let gongzuo = Gongzuo::from(gongzuo);

    Ok((StatusCode::OK, Json(json!(gongzuo))))
}
