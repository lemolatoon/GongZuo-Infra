use axum::{routing::get, Router};

use crate::db;

pub fn gongzuo_router() -> Router<db::DB> {
    Router::new().route("/", get(|| async { "Hello, world! from '/gongzuo'" }))
    // .route("/users", get(handlers::users::users))
    // .route("/register", post(handlers::register::register))
    // .route("/login", post(handlers::login::login))
    // .route("/logout", post(handlers::logout::logout))
}
