use sqlx::postgres::{PgPool, PgPoolOptions};

mod error;
mod http;

#[derive(Clone)]
pub struct APIContext {
    pub db: PgPool,
    pub encryption_key: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let db_username = std::env::var("DB_USER").unwrap_or("postgres".to_string());
    let db_password = std::env::var("DB_PASSWORD").unwrap();
    let db_host = std::env::var("DB_HOST").unwrap_or("localhost".to_string());
    let db_port = std::env::var("DB_PORT").unwrap_or("5432".to_string());
    let db_name = std::env::var("DB_NAME").unwrap_or("postgres".to_string());

    let encryption_key = std::env::var("OPENSSL_KEY").clone().to_owned().unwrap();

    let connection_url =
        format!("postgres://{db_username}:{db_password}@{db_host}:{db_port}/{db_name}");
    let database = PgPoolOptions::new()
        .max_connections(50)
        .connect(&connection_url)
        .await
        .unwrap();
    sqlx::migrate!().run(&database).await.unwrap();

    http::serve(APIContext {
        db: database,
        encryption_key,
    })
    .await?;

    Ok(())
}
