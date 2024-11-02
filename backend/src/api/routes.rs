use actix_web::{web, Scope};

use crate::api::handlers::{
    get_blockchain_status, submit_transaction, fetch_transaction, register_user, get_user_data,
};

// Function to configure API routes
pub fn api_routes() -> Scope {
    web::scope("/api")
        .route("/status", web::get().to(get_blockchain_status))
        .route("/transaction", web::post().to(submit_transaction))
        .route("/transaction/{id}", web::get().to(fetch_transaction))
        .route("/user/register", web::post().to(register_user))
        .route("/user/{id}", web::get().to(get_user_data))
}


