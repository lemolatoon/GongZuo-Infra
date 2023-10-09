use core::panic;

use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use sqlx::Postgres;

use crate::util::timezone::into_jst;

#[derive(
    Debug, Serialize_repr, Deserialize_repr, sqlx::Type, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,
)]
#[repr(i32)]
pub enum ContentKind {
    Work = 0,
    NotWork = 1,
}

impl From<i32> for ContentKind {
    fn from(value: i32) -> Self {
        match value {
            0 => ContentKind::Work,
            1 => ContentKind::NotWork,
            _ => panic!("Invalid content kind"),
        }
    }
}

impl From<ContentKind> for i32 {
    fn from(value: ContentKind) -> Self {
        match value {
            ContentKind::Work => 0,
            ContentKind::NotWork => 1,
        }
    }
}

#[derive(sqlx::FromRow, Deserialize, Debug)]
pub struct GongzuoRaw {
    pub id: i32,
    pub user_id: i32,
    pub content_id: i32,
    pub started_at: NaiveDateTime,
    pub ended_at: Option<NaiveDateTime>,
    pub content_kind: ContentKind,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Gongzuo {
    pub id: i32,
    pub user_id: i32,
    pub content_id: i32,
    pub started_at: DateTime<FixedOffset>,
    pub ended_at: Option<DateTime<FixedOffset>>,
    pub content_kind: ContentKind,
    pub content: String,
}

impl From<GongzuoRaw> for Gongzuo {
    fn from(value: GongzuoRaw) -> Self {
        let GongzuoRaw {
            id,
            user_id,
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
            user_id,
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

pub struct GongzuoPayload {
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub content_kind: ContentKind,
    pub content: String,
}

#[axum::async_trait]
pub trait GongzuoHandlerTrait {
    async fn all_gongzuos(&self) -> anyhow::Result<Vec<GongzuoRaw>>;
    async fn gongzuos_by_user_id(&self, user_id: i32) -> anyhow::Result<Vec<GongzuoRaw>>;
    async fn create_gongzuo(&self, user_id: i32, payload: GongzuoPayload) -> anyhow::Result<i32>;
    async fn update_gongzuo(
        &self,
        gongzuo_id: i32,
        user_id: i32,
        payload: GongzuoPayload,
    ) -> anyhow::Result<Result<(), String>>;
    async fn delete_gongzuo(&self, id: i32, user_id: i32) -> anyhow::Result<Result<(), String>>;
    async fn gongzuo_by_gongzuo_id(&self, gongzuo_id: i32) -> anyhow::Result<Option<GongzuoRaw>>;
    async fn gongzuo_at(
        &self,
        user_id: i32,
        at: DateTime<Utc>,
    ) -> anyhow::Result<Option<GongzuoRaw>>;
}

impl<'a> GongzuoHandler<'a> {
    pub fn new(pool: &'a sqlx::Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[axum::async_trait]
impl GongzuoHandlerTrait for GongzuoHandler<'_> {
    async fn all_gongzuos(&self) -> anyhow::Result<Vec<GongzuoRaw>> {
        let gongzuos = sqlx::query_as!(
            GongzuoRaw,
            r#"
            SELECT
                gongzuo.id AS id,
                contents.id AS content_id,
                user_id,
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
            JOIN
                users
            ON
                gongzuo.user_id = users.id
            WHERE
                users.is_admin = false
            "#
        )
        .fetch_all(self.pool)
        .await?;

        Ok(gongzuos)
    }

    async fn gongzuos_by_user_id(&self, user_id: i32) -> anyhow::Result<Vec<GongzuoRaw>> {
        let gongzuos = sqlx::query_as!(
            GongzuoRaw,
            r#"
            SELECT
                gongzuo.id AS id,
                contents.id AS content_id,
                user_id,
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

    async fn gongzuo_by_gongzuo_id(&self, gongzuo_id: i32) -> anyhow::Result<Option<GongzuoRaw>> {
        let gongzuo = sqlx::query_as!(
            GongzuoRaw,
            r#"
            SELECT
                gongzuo.id AS id,
                contents.id AS content_id,
                user_id,
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
                gongzuo.id = $1
            "#,
            gongzuo_id
        )
        .fetch_optional(self.pool)
        .await?;

        Ok(gongzuo)
    }

    async fn create_gongzuo(&self, user_id: i32, payload: GongzuoPayload) -> anyhow::Result<i32> {
        let GongzuoPayload {
            started_at,
            ended_at,
            content_kind,
            content,
        } = payload;

        let contents = sqlx::query!(
            r#"
            SELECT
                id
            FROM
                contents
            WHERE
                content_kind = $1
            AND
                content = $2
            "#,
            i32::from(content_kind),
            content
        )
        .fetch_all(self.pool)
        .await?;

        let mut transaction = self.pool.begin().await?;

        let content_id = if contents.is_empty() {
            sqlx::query!(
                r#"
                INSERT INTO
                    contents (content_kind, content)
                VALUES
                    ($1, $2)
                RETURNING
                    id
                "#,
                content_kind as i32,
                content
            )
            .fetch_one(&mut *transaction)
            .await?
            .id
        } else {
            contents[0].id
        };

        let gongzuo_id = sqlx::query!(
            r#"
            INSERT INTO
                gongzuo (user_id, content_id, started_at, ended_at)
            VALUES
                ($1, $2, $3, $4)
            RETURNING
                id
            "#,
            user_id,
            content_id,
            started_at.naive_utc(),
            ended_at.map(|ended_at| ended_at.naive_utc())
        )
        .fetch_one(&mut *transaction)
        .await?
        .id;

        transaction.commit().await?;

        Ok(gongzuo_id)
    }

    async fn update_gongzuo(
        &self,
        gongzuo_id: i32,
        user_id: i32,
        payload: GongzuoPayload,
    ) -> anyhow::Result<Result<(), String>> {
        let GongzuoPayload {
            started_at,
            ended_at,
            content_kind,
            content,
        } = payload;

        let gongzuo_between = sqlx::query!(
            r#"
            SELECT
                id
            FROM
                gongzuo
            WHERE
                id != $1
            AND
                user_id = $2
            AND
                started_at <= $3
            AND
                (ended_at IS NULL OR ended_at > $3)
            "#,
            gongzuo_id,
            user_id,
            started_at.naive_utc()
        )
        .fetch_optional(self.pool)
        .await?;

        if gongzuo_between.is_some() {
            return Ok(Err(String::from(
                "Gongzuo already exists during the period.",
            )));
        }

        let content_ids = sqlx::query!(
            r#"
            SELECT
                id
            FROM
                contents
            WHERE
                content_kind = $1
            AND
                content = $2
            "#,
            i32::from(content_kind),
            content
        )
        .fetch_all(self.pool)
        .await?;

        let mut transaction = self.pool.begin().await?;

        let content_id = if content_ids.is_empty() {
            sqlx::query!(
                r#"
                INSERT INTO
                    contents (content_kind, content)
                VALUES
                    ($1, $2)
                RETURNING
                    id
                "#,
                content_kind as i32,
                content
            )
            .fetch_one(&mut *transaction)
            .await?
            .id
        } else {
            content_ids[0].id
        };

        // user_id が一致しなければrollbackする。
        let Some(fetched_user_id) = sqlx::query!(
            r#"
            UPDATE
                gongzuo
            SET
                content_id = $1,
                started_at = $2,
                ended_at = $3
            WHERE
                id = $4
            RETURNING
                user_id
            "#,
            content_id,
            started_at.naive_utc(),
            ended_at.map(|ended_at| ended_at.naive_utc()),
            gongzuo_id
        )
        .fetch_optional(&mut *transaction)
        .await?
        .map(|r| r.user_id) else {
            transaction.rollback().await?;
            return Ok(Err(format!("Gongzuo {} not found", gongzuo_id)));
        };

        if fetched_user_id != user_id {
            transaction.rollback().await?;
            return Ok(Err(String::from("User id mismatch")));
        }

        transaction.commit().await?;

        Ok(Ok(()))
    }

    async fn delete_gongzuo(
        &self,
        gongzuo_id: i32,
        user_id: i32,
    ) -> anyhow::Result<Result<(), String>> {
        println!(
            "delete_gongzuo: gongzuo_id: {}, user_id: {}",
            gongzuo_id, user_id
        );
        let mut transaction = self.pool.begin().await?;

        // user_id が一致しなければrollbackする。
        let Some(fetched_user_id) = sqlx::query!(
            r#"
            DELETE
            FROM
                gongzuo
            WHERE
                id = $1
            RETURNING
                user_id
            "#,
            gongzuo_id,
        )
        .fetch_optional(&mut *transaction)
        .await?
        .map(|r| r.user_id) else {
            transaction.rollback().await?;
            return Ok(Err(format!("Gongzuo {} not found", gongzuo_id)));
        };

        if fetched_user_id != user_id {
            transaction.rollback().await?;
            return Ok(Err("User id mismatch".to_string()));
        }

        transaction.commit().await?;

        Ok(Ok(()))
    }

    async fn gongzuo_at(
        &self,
        user_id: i32,
        at: DateTime<Utc>,
    ) -> anyhow::Result<Option<GongzuoRaw>> {
        let row = sqlx::query_as!(
            GongzuoRaw,
            r#"
            SELECT
                gongzuo.id AS id,
                contents.id AS content_id,
                user_id,
                started_at,
                ended_at,
                content_kind,
                content
            FROM gongzuo
            JOIN
                contents
            ON
                gongzuo.content_id = contents.id
            WHERE user_id = $1 AND started_at <= $2 AND (ended_at IS NULL OR ended_at > $2)
            "#,
            user_id,
            at.naive_utc(),
        )
        .fetch_optional(self.pool)
        .await?;

        Ok(row)
    }
}
