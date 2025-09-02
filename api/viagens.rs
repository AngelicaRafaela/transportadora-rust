use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use transportadora::{db::get_pool, models::Viagem};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
struct NovaViagem {
    origem_cidade: String,
    destino_cidade: String,
    motorista_id: Uuid,
    caminhao_id: Uuid,
    cliente_id: Uuid,
    valor_frete: Option<f64>,
}

#[tokio::main]
async fn main() -> Result<(), Error> { run(handler).await }

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let pool = get_pool().await;

    match req.method().as_str() {
        "GET" => {
            let viagens: Vec<Viagem> = sqlx::query_as::<_, Viagem>("SELECT * FROM viagens ORDER BY created_at DESC LIMIT 50")
                .fetch_all(pool).await?;

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Access-Control-Allow-Origin", "*")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&viagens)?.into())?)
        }
        "POST" => {
            let bytes = hyper::body::to_bytes(req.into_body()).await?;
            let data: NovaViagem = serde_json::from_slice(&bytes)?;

            let id = Uuid::new_v4();
            let rec = sqlx::query_as::<_, Viagem>(
                r#"
                INSERT INTO viagens
                  (id, origem_cidade, destino_cidade, status, motorista_id, caminhao_id, cliente_id, valor_frete)
                VALUES ($1,$2,$3,'aberta',$4,$5,$6,$7)
                RETURNING *
                "#
            )
            .bind(id)
            .bind(&data.origem_cidade)
            .bind(&data.destino_cidade)
            .bind(data.motorista_id)
            .bind(data.caminhao_id)
            .bind(data.cliente_id)
            .bind(data.valor_frete)
            .fetch_one(pool).await?;

            Ok(Response::builder()
                .status(StatusCode::CREATED)
                .header("Access-Control-Allow-Origin", "*")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&rec)?.into())?)
        }
        "OPTIONS" => {
            // CORS preflight simples
            Ok(Response::builder()
                .status(StatusCode::NO_CONTENT)
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Methods", "GET,POST,OPTIONS")
                .header("Access-Control-Allow-Headers", "Content-Type")
                .body(Body::Empty)?)
        }
        _ => Ok(Response::builder().status(StatusCode::METHOD_NOT_ALLOWED).body(Body::Empty)?),
    }
}
