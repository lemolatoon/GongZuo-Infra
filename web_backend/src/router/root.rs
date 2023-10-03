use axum::{
    routing::{get, post},
    Router,
};

use crate::{db, handlers};

pub fn app_router(db: db::DB) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, world! from '/'" }))
        .route("/users", get(handlers::users::users))
        .route("/register", post(handlers::register::register))
        .route("/login", post(handlers::login::login))
        .route("/logout", post(handlers::logout::logout))
        .with_state(db)
}
