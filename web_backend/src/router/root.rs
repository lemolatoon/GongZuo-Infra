use axum::{
    http::{header::CONTENT_TYPE, HeaderValue, Method},
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;

use crate::{db, handlers, router};

pub fn app_router(db: db::DB) -> Router {
    let allowed_orgins: Vec<HeaderValue> =
        ["http://localhost:3000", "https://gongzuo-one.vercel.app"]
            .into_iter()
            .map(|o| o.parse().unwrap())
            .collect();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(allowed_orgins)
        .allow_credentials(true)
        .allow_headers([CONTENT_TYPE]);

    Router::new()
        .route("/", get(|| async { "Hello, world! from '/'" }))
        .route("/users", get(handlers::users::users))
        .route("/register", post(handlers::register::register))
        .route("/login", post(handlers::login::login))
        .route("/logout", post(handlers::logout::logout))
        .route("/me", get(handlers::users::me))
        .nest("/gongzuo", router::gongzuo::gongzuo_router())
        .with_state(db)
        .layer(cors)
}
