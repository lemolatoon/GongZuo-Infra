pub mod gongzuo;
pub mod user;

use sqlx::{Pool, Postgres};

use self::{gongzuo::GongzuoHandlerTrait, user::UserHandlerTrait};

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

    pub fn gongzuo_handler(&self) -> impl GongzuoHandlerTrait + '_ {
        gongzuo::GongzuoHandler::new(&self.pool)
    }
}
