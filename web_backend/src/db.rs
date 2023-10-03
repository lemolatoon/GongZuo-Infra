pub mod user;

use sqlx::{Pool, Postgres};

use self::user::UserHandlerTrait;

#[derive(Clone)]
pub struct DB {
    pool: Pool<Postgres>,
}

impl DB {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub fn user_handler(&self) -> impl UserHandlerTrait + '_ {
        user::UserHandler::new(&self.pool)
    }
}
