use serde_json::Value;
use std::collections::HashMap;

// A utility function to convert a JSON object into a HashMap
pub fn json_to_map(json: &Value) -> HashMap<String, String> {
    let mut map = HashMap::new();
    if let Some(obj) = json.as_object() {
        for (key, value) in obj {
            if let Some(value_str) = value.as_str() {
                map.insert(key.clone(), value_str.to_string());
            }
        }
    }
    map
}

// A utility function to generate a random string of a given length
pub fn generate_random_string(length: usize) -> String {
    use rand::{distributions::Alphanumeric, Rng};
    let rng = rand::thread_rng();
    rng.sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

// A utility function to handle error messages more cleanly
pub fn format_error_message(error: &str) -> String {
    format!("An error occurred: {}", error)
}


