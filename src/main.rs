use actix_web::{App, HttpServer};
use sqlx::postgres::PgPoolOptions;

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL não configurada");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Falha ao conectar ao banco");

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .configure(api::routes::config)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
