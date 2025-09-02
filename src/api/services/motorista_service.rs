use crate::api::models::motorista::Motorista;
use sqlx::PgPool;

pub async fn listar_motoristas(pool: &PgPool) -> Result<Vec<Motorista>, sqlx::Error> {
    let motoristas = sqlx::query_as::<_, Motorista>("SELECT * FROM motoristas")
        .fetch_all(pool)
        .await?;
    Ok(motoristas)
}
