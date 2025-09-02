use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use transportadora::db::get_pool;

#[tokio::main]
async fn main() -> Result<(), Error> { run(handler).await }

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    // Testa conexão
    let pool = get_pool().await;
    let row: (i64,) = sqlx::query_as("SELECT 1").fetch_one(pool).await?;
    let ok = row.0 == 1;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::Text(format!(r#"{{"ok": {ok}}}"#)))?)
}
