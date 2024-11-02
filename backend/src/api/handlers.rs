use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::services::{blockchain_service, user_service};

// Handler to get the current status of the blockchain
pub async fn get_blockchain_status() -> impl Responder {
    match blockchain_service::get_status().await {
        Ok(status) => HttpResponse::Ok().json(status),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// Handler to submit a transaction to the blockchain
pub async fn submit_transaction(
    pool: web::Data<PgPool>,
    transaction_data: web::Json<String>,
) -> impl Responder {
    match blockchain_service::submit_transaction(&transaction_data).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// Handler to fetch a transaction from the blockchain
pub async fn fetch_transaction(
    pool: web::Data<PgPool>,
    id: web::Path<String>,
) -> impl Responder {
    match blockchain_service::fetch_transaction(&id).await {
        Ok(transaction) => HttpResponse::Ok().json(transaction),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// Handler to register a new user in the system
pub async fn register_user(
    pool: web::Data<PgPool>,
    user_data: web::Json<user_service::User>,
) -> impl Responder {
    match user_service::register_user(pool.get_ref(), &user_data).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// Handler to get user data
pub async fn get_user_data(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
) -> impl Responder {
    match user_service::get_user_by_id(pool.get_ref(), id.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}


