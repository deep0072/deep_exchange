use sqlx::{Postgres, postgres::PgPoolOptions};

pub type Pool = sqlx::Pool<Postgres>;

pub async fn init__pool(database_url: &str) -> Pool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("db failed to connect")
}
