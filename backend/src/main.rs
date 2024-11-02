mod api;
mod db;
mod services;
mod utils;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use utils::logging::initialize_logging;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();
    
    // Use unwrap_or_else to handle the default server address
    let server_address = env::var("SERVER_ADDRESS").unwrap_or_else(|err| {
        log::warn!("Could not find SERVER_ADDRESS in environment variables: {}", err);
        "127.0.0.1:8080".to_string()
    });

    // Initialize logging
    initialize_logging();
    log::info!("Starting server at {}", server_address);

    // Create the Actix web server
    HttpServer::new(|| {
        App::new()
            .configure(api::routes::init_routes) // Configure API routes
    })
    .bind(&server_address)?
    .run()
    .await
}

