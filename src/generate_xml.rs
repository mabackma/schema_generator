use quick_xml::Writer;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use serde_json::Value;
use std::collections::HashMap;
use std::io::Cursor;

pub fn create_xml_element(json_data: &Value, writer: &mut Writer<Cursor<Vec<u8>>>, parent_tag: &str) {
    match json_data {
        // Handle objects, which may include both attributes and nested elements
        Value::Object(map) => {
            let mut element = BytesStart::new(parent_tag);

            // Extract attributes
            let attributes: HashMap<_, _> = map
                .iter()
                .filter(|(key, _)| key.starts_with('@'))
                .map(|(key, value)| (&key[1..], value))
                .collect();

            // Add attributes to the element
            for (key, value) in &attributes {
                if let Some(value_str) = value.as_str() {
                    element.push_attribute((*key, value_str));
                }
            }

            // Write start tag with attributes, if any
            if !attributes.is_empty() {
                writer
                    .write_event(Event::Start(element.to_owned()))
                    .expect("Unable to write start tag");
            }

            // Process key-value pairs
            for (key, value) in map {
                // Write self-closing tag if the object is empty
                if value.is_object() && value.as_object().unwrap().is_empty() {
                    writer
                        .write_event(Event::Empty(BytesStart::new(key)))
                        .expect("Unable to write self-closing tag");
                    continue;
                }

                // Skip attributes
                if key.starts_with('@') {
                    continue;
                } else { 				  
					// Reset the element for the next iteration
					element = BytesStart::new(key);
                    
                    // Write the start tag if the value is not an attribute or an array with a first key as an attribute
                    if !(is_attribute_key(value) || is_array_with_attribute_key(value)) {
                        writer
                            .write_event(Event::Start(element.to_owned()))
                            .expect("Unable to write start tag");
                    }

					// Recursively process nested elements
					create_xml_element(value, writer, key);
					
                    // Write the closing tag if the value is not an array
                    if !value.is_array() {
                        writer
                            .write_event(Event::End(BytesEnd::new(key)))
                            .expect("Unable to write end tag");
                    }
                }
            }
        },
        // Handle arrays by processing each item inside the array
        Value::Array(arr) => {
            for (i, value) in arr.iter().enumerate() {
                // Get the first key of the object 
                if value.is_object() {
                    let first_key = value.as_object().unwrap().keys().next().unwrap();

                    // Write the start tag for all non-attribute elements, skipping the first one
                    if !first_key.starts_with('@') && i > 0 {
                        writer
                            .write_event(Event::Start(BytesStart::new(parent_tag)))
                            .expect("Unable to write start tag"); 
                    }
                }
                
                // Process each element of the array as a separate XML tag
                create_xml_element(value, writer, parent_tag);

                // Write the closing tag
                writer
                    .write_event(Event::End(BytesEnd::new(parent_tag)))
                    .expect("Unable to write end tag");
            }
        },
        // Handle strings as text content
        Value::String(s) => {
            writer
                .write_event(Event::Text(BytesText::new(s)))
                .expect("Unable to write text");
        },
        _ => {}  // Skip unsupported types (e.g., Null)
    }
}

// Check if the first key of the object is an attribute
fn is_attribute_key(value: &Value) -> bool {
    value.is_object()
        && value.as_object()
            .unwrap()
            .keys()
            .next()
            .map(|key| key.starts_with('@'))
            .unwrap_or(false)
}

// Check if the first key of the first object in array is an attribute
fn is_array_with_attribute_key(value: &Value) -> bool {
    value.is_array()
        && value.as_array()
            .unwrap()
            .first()
            .map(|v| is_attribute_key(v))
            .unwrap_or(false)
}