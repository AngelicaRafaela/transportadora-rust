use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Motorista {
    pub id: Uuid,
    pub nome: String,
    pub cnh: String,
    pub telefone: Option<String>,
    pub ativo: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Caminhao {
    pub id: Uuid,
    pub placa: String,
    pub marca: Option<String>,
    pub modelo: Option<String>,
    pub ano: Option<i32>,
    pub cor: Option<String>,
    pub placa_carreta: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Cliente {
    pub id: Uuid,
    pub nome_razao: String,
    pub cpf: Option<String>,
    pub cnpj: Option<String>,
    pub email: Option<String>,
    pub telefone: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Viagem {
    pub id: Uuid,
    pub origem_cidade: String,
    pub destino_cidade: String,
    pub data_partida: Option<chrono::NaiveDate>,
    pub data_chegada: Option<chrono::NaiveDate>,
    pub status: String,
    pub motorista_id: Uuid,
    pub caminhao_id: Uuid,
    pub cliente_id: Uuid,
    pub distancia_km: Option<f64>,
    pub valor_frete: Option<f64>,
    pub created_at: DateTime<Utc>,
}
