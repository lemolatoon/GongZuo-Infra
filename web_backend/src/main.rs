pub mod db;
pub mod error;
pub mod handlers;
pub mod password;
pub mod router;
pub mod session;

use std::net::SocketAddr;

use axum::{self};
use db::user::UserHandlerTrait;
use once_cell::sync::Lazy;
use sqlx::postgres::PgPoolOptions;

use crate::router::root::app_router;

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

    let is_registered = db
        .user_handler()
        .ensure_admin_user_is_registered("admin")
        .await
        .unwrap();
    if !is_registered {
        panic!("Admin user is not registered");
    }

    let app = app_router(db);

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
