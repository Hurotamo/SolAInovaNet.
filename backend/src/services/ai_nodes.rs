
use crate::db::ai_nodes_db;
use crate::utils::logging::log_info;
use serde_json::Value;

// Function to register a new AI node
pub fn register_ai_node(node_data: &Value) -> Result<String, String> {
    let node_id = ai_nodes_db::store_ai_node_data(node_data);
    log_info(&format!("AI node {} registered", node_id));
    Ok(node_id)
}

// Function to update AI node status
pub fn update_ai_node_status(node_id: &str, status: &str) -> Result<(), String> {
    ai_nodes_db::update_status(node_id, status);
    log_info(&format!("AI node {} status updated to {}", node_id, status));
    Ok(())
}

// Function to retrieve AI node metrics
pub fn get_ai_node_metrics(node_id: &str) -> Value {
    ai_nodes_db::fetch_metrics(node_id)
}

