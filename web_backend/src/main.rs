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

    let users = sqlx::query!("SELECT * FROM users")
        .fetch_all(&pool)
        .await
        .unwrap();

    println!("users: {:?}", &users);
    if let Some(user) = users.first() {
        println!("id: {}, username: {}", user.id, user.username);
    }
}
