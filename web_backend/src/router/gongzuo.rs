use axum::{routing::get, Router};

use crate::{db, handlers};

pub fn gongzuo_router() -> Router<db::DB> {
    Router::new()
        .route("/", get(|| async { "Hello, world! from '/gongzuo'" }))
        .route("/gongzuos", get(handlers::gongzuo::gongzuos))
    // .route("/register", post(handlers::register::register))
    // .route("/login", post(handlers::login::login))
    // .route("/logout", post(handlers::logout::logout))
}
