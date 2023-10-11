pub mod db;
pub mod error;
pub mod handlers;
pub mod password;
pub mod router;
pub mod session;
pub mod util;

use std::{
    net::{IpAddr, SocketAddr},
    path::PathBuf,
};

use axum::{self};
use axum_server::tls_rustls::RustlsConfig;
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
    let host: IpAddr = std::env::var("HOST")
        .map(|i| i.parse().unwrap())
        .unwrap_or([127, 0, 0, 1].into());

    let addr = SocketAddr::from((host, port));

    let tls = std::env::var("TLS").ok().is_some();

    if tls {
        let config = RustlsConfig::from_pem_file(
            std::env::var("CERT_PATH")
                .map(|path| PathBuf::from(path))
                .unwrap_or(
                    PathBuf::from("/")
                        .join("etc")
                        .join("letsencrypt")
                        .join("live")
                        .join("lemolatoon.ddns.net")
                        .join("cert.pem"),
                ),
            std::env::var("PRIVATE_KEY_PATH")
                .map(|path| PathBuf::from(path))
                .unwrap_or(
                    PathBuf::from("/")
                        .join("etc")
                        .join("letsencrypt")
                        .join("live")
                        .join("lemolatoon.ddns.net")
                        .join("privkey.pem"),
                ),
        )
        .await
        .unwrap();
        println!("Listening on {}", &addr);
        axum_server::bind_rustls(addr, config)
            .serve(app.into_make_service())
            .await
            .unwrap();
        return;
    }

    // run it with hyper
    println!("Listening on {}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
