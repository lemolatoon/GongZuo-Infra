pub mod db;

use std::net::SocketAddr;

use axum::{self, response::IntoResponse, routing::get, Extension, Json, Router};
use chrono::NaiveDateTime;
use db::{user::User, DB};
use dotenvy;
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
        .layer(Extension(db));

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub async fn users(Extension(db): Extension<DB>) -> impl IntoResponse {
    let mut users = db.users().await;
    // dummy user
    users.push(User {
        id: 1,
        username: "username".to_string(),
        password: "password".to_string(),
        salt: "salt".to_string(),
        created_at: NaiveDateTime::from_timestamp_opt(0, 0).unwrap(),
    });
    println!("users: {:?}", users);
    Json(users)
}
