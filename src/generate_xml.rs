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
            let mut attributes = HashMap::new();

            // Look for attributes in the object
            for (key, value) in map {
                if key.starts_with('@') {
                    attributes.insert(&key[1..], value);
                    continue;
                }
            } 

            if !attributes.is_empty() {
                // Write the start tag with attributes
                for (key, value) in &attributes {
                    element.push_attribute((*key, value.as_str().unwrap()));
                }

                writer
                    .write_event(Event::Start(element.to_owned()))
                    .expect("Unable to write start tag");

                attributes.clear();
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

                    // Check if the first key of the object is an attribute
                    let is_attribute_key = |value: &Value| {
                        value.is_object()
                            && value.as_object()
                                .unwrap()
                                .keys()
                                .next()
                                .map(|key| key.starts_with('@'))
                                .unwrap_or(false)
                    };
                    
                    // Check if the first key of the first object in array is an attribute
                    let is_array_with_attribute_key = |value: &Value| {
                        value.is_array()
                            && value.as_array()
                                .unwrap()
                                .first()
                                .map(|v| is_attribute_key(v))
                                .unwrap_or(false)
                    };
                    
                    // Write the start tag if the value is not an attribute or an array with an attribute key
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
            let mut first_element = true;

            for value in arr {
                // Get the first key of the object 
                if value.is_object() || value.is_array() {
                    let first_key = value.as_object().unwrap().keys().next().unwrap();
                    
                    // Write the start tag for all elements except the first one
                    if !first_key.starts_with('@') && !first_element {
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

                first_element = false;
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
