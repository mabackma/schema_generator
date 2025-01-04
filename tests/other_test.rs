use schema_generator::json_utils::{json_keys_to_lowercase, extract_prefixes};

use serde_json::json;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_keys_to_lowercase() {

        // Prepare the input JSON object
        let input = serde_json::json!({
            "FooBar": {
                "@KeyName": "value",
                "NestedObject": {
                    "SomeKey": "value"
                }
            },
            "AnotherField": "value"
        });

        // Expected output with transformed keys
        let expected = serde_json::json!({
            "fooBar": {
                "__KeyName": "value",
                "nestedObject": {
                    "someKey": "value"
                }
            },
            "anotherField": "value"
        });
        
        let result = json_keys_to_lowercase(&input);

        // Compare the result with the expected output
        assert_eq!(result, expected);
    }

    #[test]
    fn test_extract_prefixes() {

        // Create a JSON object with "__xmlns:" keys
        let json_data = json!({
            "__xmlns:ns1": "http://example.com/2021/05/01/schema1",
            "__xmlns:ns2": "http://example.com/2021/06/01/schema2",
            "otherKey": "value"
        });

        // Expected result
        let expected = {
            let mut map = HashMap::new();
            map.insert("schema1".to_string(), "ns1".to_string());
            map.insert("schema2".to_string(), "ns2".to_string());
            map
        };

        // Call the function with the test data
        let result = extract_prefixes(&json_data);

        // Assert the result matches the expected output
        assert_eq!(result, expected);
    }

    #[test]
    fn test_extract_prefixes_no_xmlns() {
        
        // Test with a JSON object that does not contain __xmlns: keys
        let json_data = json!({
            "someKey": "value",
            "anotherKey": "anotherValue"
        });

        // The result should be an empty map
        let expected = HashMap::new();
        
        let result = extract_prefixes(&json_data);

        assert_eq!(result, expected);
    }
}
