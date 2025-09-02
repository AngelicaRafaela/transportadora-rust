use std::env;
use sqlx::{Pool, Postgres};
use tokio::sync::OnceCell;

static POOL: OnceCell<Pool<Postgres>> = OnceCell::const_new();

pub async fn get_pool() -> &'static Pool<Postgres> {
    POOL.get_or_init(|| async {
        let url = env::var("DATABASE_URL").expect("DATABASE_URL não definida");
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(&url)
            .await
            .expect("Falha ao conectar no Postgres")
    }).await
}
