use actix_web::{get, web, HttpResponse};
use crate::api::services::motorista_service;

#[get("/motoristas")]
async fn get_motoristas(pool: web::Data<sqlx::PgPool>) -> HttpResponse {
    match motorista_service::listar_motoristas(&pool).await {
        Ok(motoristas) => HttpResponse::Ok().json(motoristas),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
