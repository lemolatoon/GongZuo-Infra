use chrono::{DateTime, FixedOffset, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::Postgres;

use crate::util::timezone::into_jst;

#[derive(sqlx::FromRow, Deserialize, Debug)]
pub struct GongzuoRaw {
    pub id: i32,
    pub content_id: i32,
    pub started_at: NaiveDateTime,
    pub ended_at: Option<NaiveDateTime>,
    pub content_kind: i32,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Gongzuo {
    pub id: i32,
    pub content_id: i32,
    pub started_at: DateTime<FixedOffset>,
    pub ended_at: Option<DateTime<FixedOffset>>,
    pub content_kind: i32,
    pub content: String,
}

impl From<GongzuoRaw> for Gongzuo {
    fn from(value: GongzuoRaw) -> Self {
        let GongzuoRaw {
            id,
            content_id,
            started_at,
            ended_at,
            content_kind,
            content,
        } = value;

        let started_at = into_jst(started_at);
        let ended_at = ended_at.map(into_jst);

        Gongzuo {
            id,
            content_id,
            started_at,
            ended_at,
            content_kind,
            content,
        }
    }
}

pub struct GongzuoHandler<'a> {
    pool: &'a sqlx::Pool<Postgres>,
}

#[axum::async_trait]
pub trait GongzuoHandlerTrait {
    async fn gongzuos_by_user_id(&self, user_id: i32) -> anyhow::Result<Vec<GongzuoRaw>>;
}

impl<'a> GongzuoHandler<'a> {
    pub fn new(pool: &'a sqlx::Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[axum::async_trait]
impl GongzuoHandlerTrait for GongzuoHandler<'_> {
    async fn gongzuos_by_user_id(&self, user_id: i32) -> anyhow::Result<Vec<GongzuoRaw>> {
        let gongzuos = sqlx::query_as!(
            GongzuoRaw,
            r#"
            SELECT
                gongzuo.id AS id,
                contents.id AS content_id,
                started_at,
                ended_at,
                content_kind,
                content
            FROM
                gongzuo
            JOIN
                contents
            ON
                gongzuo.content_id = contents.id
            WHERE
                gongzuo.user_id = $1
            "#,
            user_id
        )
        .fetch_all(self.pool)
        .await?;

        Ok(gongzuos)
    }
}
