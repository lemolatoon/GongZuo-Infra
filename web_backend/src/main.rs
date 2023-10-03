pub mod db;
pub mod error;
pub mod handlers;
pub mod password;
pub mod session;

use std::net::SocketAddr;

use axum::{
    self,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use db::DB;
use dotenvy;
use error::Result;
use once_cell::sync::Lazy;
use sqlx::postgres::PgPoolOptions;

static DATABASE_URL: Lazy<String> = Lazy::new(|| {
    dotenvy::dotenv().unwrap();

    std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set. Maybe you forgot to run `cargo make dotenv`")
});

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&DATABASE_URL)
        .await
        .unwrap();

    let db = db::DB::new(pool);

    let app = Router::new()
        .route("/", get(|| async { "Hello, world! from '/'" }))
        .route("/users", get(users))
        .route("/register", post(handlers::register::register))
        .route("/login", post(handlers::login::login))
        .layer(Extension(db));

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub async fn users(Extension(db): Extension<DB>) -> Result<impl IntoResponse> {
    let users = db.users().await?;
    Ok(Json(users))
}
