pub mod db;
pub mod error;
pub mod handlers;
pub mod password;
pub mod session;

use std::net::SocketAddr;

use axum::{
    self,
    extract::State,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use db::DB;
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

    let is_registered = db.ensure_admin_user_is_registered("admin").await.unwrap();
    if !is_registered {
        panic!("Admin user is not registered");
    }

    let app = Router::new()
        .route("/", get(|| async { "Hello, world! from '/'" }))
        .route("/users", get(users))
        .route("/register", post(handlers::register::register))
        .route("/login", post(handlers::login::login))
        .route("/logout", post(handlers::logout::logout))
        .with_state(db);

    let port = std::env::var("PORT")
        .map_or(None, |p| p.parse().ok())
        .unwrap_or(3001);
    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("Listening on {}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub async fn users(State(db): State<DB>) -> Result<impl IntoResponse> {
    let users = db.users().await?;
    Ok(Json(users))
}
