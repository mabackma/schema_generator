use crate::string_utils::lowercase_word;

use std::collections::HashMap;
use regex::Regex;

// Converts the keys of a JSON object to lowercase and replaces @ with __
pub fn json_keys_to_lowercase(json: &serde_json::Value) -> serde_json::Value {
    match json {
        serde_json::Value::Object(map) => {
            let mut new_map = serde_json::Map::new();
            for (key, value) in map {
                new_map.insert(lowercase_word(&key), json_keys_to_lowercase(value));
            }
            serde_json::Value::Object(new_map)
        },
        serde_json::Value::Array(vec) => {
            let new_vec: Vec<serde_json::Value> = vec.iter().map(|v| json_keys_to_lowercase(v)).collect();
            serde_json::Value::Array(new_vec)
        },
        _ => json.clone(),
    }
}

// Create a map of prefixes from the root element of the JSON data
pub fn extract_prefixes(json_data: &serde_json::Value) -> HashMap<String, String> {
    let mut prefixes: HashMap<String, String> = HashMap::new();

    match json_data {
        serde_json::Value::Object(map) => {
            for (key, value) in map {
                if key.starts_with("__xmlns:") {
                    let prefix = key.split(':').last().unwrap().to_string();
                    let struct_string = value.as_str().unwrap().to_string();

                    // Extract the namespace from the struct string
                    let re = Regex::new(r"/\d{4}/\d{2}/\d{2}").unwrap();
                    let namespace = re.replace(&struct_string, "").to_string();

                    // Extract the last segment of the namespace
                    let last_segment = namespace.split('/').last().unwrap().to_string();
                    let formatted_namespace = lowercase_word(&last_segment);

                    prefixes.insert(formatted_namespace, prefix);
                }
            }
        },
        _ => {}
    }

    prefixes
}
