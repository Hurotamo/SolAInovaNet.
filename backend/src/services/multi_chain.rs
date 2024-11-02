use crate::utils::logging::log_info;
use crate::db::multi_chain_db;
use serde_json::Value;

// Function to initialize multi-blockchain interactions
pub fn initialize_multi_chain_service(chains: &[String]) {
    for chain in chains {
        log_info(&format!("Multi-chain service initialized for: {}", chain));
    }
}

// Function to handle cross-chain transactions
pub fn handle_cross_chain_transaction(data: &Value) -> Result<String, String> {
    let tx_id = multi_chain_db::store_cross_chain_transaction(data);
    log_info(&format!("Cross-chain transaction {} processed", tx_id));
    Ok(tx_id)
}

// Function to fetch details of a transaction across chains
pub fn get_cross_chain_transaction_details(tx_id: &str) -> Value {
    multi_chain_db::fetch_transaction_details(tx_id)
}


