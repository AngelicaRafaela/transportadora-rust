use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Motorista {
    pub id: i32,
    pub nome: String,
    pub cnh: String,
    pub telefone: String,
}
