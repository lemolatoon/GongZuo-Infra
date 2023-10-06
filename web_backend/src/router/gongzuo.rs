use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::{db, handlers};

pub fn gongzuo_router() -> Router<db::DB> {
    Router::new()
        .route("/", get(|| async { "Hello, world! from '/gongzuo'" }))
        .route("/gongzuos", get(handlers::gongzuo::all_ongzuos))
        .route("/delete", delete(handlers::gongzuo::delete_gongzuo))
        .route("/edit", put(handlers::gongzuo::edit_gongzuo))
        .route("/start", post(handlers::gongzuo::start_gongzuo))
        .route("/end", post(handlers::gongzuo::end_gongzuo))
        .route("/:id", get(handlers::gongzuo::gongzuo_by_id))
}
