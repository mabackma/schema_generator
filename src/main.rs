use schema_generator::generate_string::generate_structs_string;
use schema_generator::create_structs::create_structs;
use schema_generator::file_structs::ForestPropertyData as FileForestPropertyData;
use schema_generator::url_structs::ForestPropertyData as UrlForestPropertyData;

use quick_xml::Writer;
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::reader::Reader;
use quick_xml::de::from_str;
use reqwest::blocking::get;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write, Cursor};
use std::str;
use std::fs;
use std::any::type_name;

fn main() {    
    // Create structs schema from a file
    let file_xml_string = read_xml_file("forestpropertydata.xml");
    create_structs_and_save_to_file(&file_xml_string, "src/file_structs.rs");

    // Convert the file XML content to structs according to the schema
    let file_property = file_content_to_structs("forestpropertydata.xml");

    // Create structs schema from a URL
    let url = "https://avoin.metsakeskus.fi/rest/mvrest/FRStandData/v1/ByPolygon?wktPolygon=POLYGON%20((393960.156%206801453.126,%20394798.608%206801657.878,%20394930.512%206801670.111,%20395028.723%206802116.858,%20394258.945%206801929.148,%20394261.711%206801810.541,%20394091.166%206801665.961,%20393960.156%206801453.126))&stdVersion=MV1.9";
    let url_xml_string = fetch_xml_url(url).unwrap();
    create_structs_and_save_to_file(&url_xml_string, "src/url_structs.rs");

    // Convert the URL XML content to structs according to the schema
    let url_property = string_content_to_structs(&url_xml_string);

    // Convert the structs to json and save to a json file
    property_to_json(Some(file_property), Some(url_property));

    json_to_xml("file_forestpropertydata.json");
}

// Reads an XML file and returns its contents as a string
fn read_xml_file(file_name: &str) -> String {
    let mut file = File::open(file_name).unwrap();
    let mut xml_string = String::new();
    file.read_to_string(&mut xml_string).unwrap();
    xml_string
}

// Fetches an XML file from a URL and returns its contents as a string
fn fetch_xml_url(url: &str) -> Option<String> {
    match get(url) {
        Ok(resp) => {
            match resp.text() {
                Ok(text) => Some(text),
                Err(e) => {
                    println!("Error: {}", e);
                    None
                }
            }
        }
        Err(e) => {
            println!("Error: {}", e);
            None
        }
    }
}

// Creates structs from an XML string and saves them to a file
fn create_structs_and_save_to_file(xml_string: &str, file_name: &str) {
    let mut reader = Reader::from_str(&xml_string);
    
    let mut structs = create_structs(&mut reader);

    let struct_string = generate_structs_string(&mut structs);

    // Save the generated structs to a file
    let mut struct_file = File::create(file_name).unwrap();
    struct_file.write_all(&struct_string.as_bytes()).unwrap();

    println!("Structs saved to {}\n", file_name);
}

// Reads XML content from a file and converts it to structs according to the schema
fn file_content_to_structs(path: &str) -> FileForestPropertyData {
    let xml = fs::read_to_string(path).expect("Could not read the XML file");
    from_str(&xml).expect("Could not parse the XML")
}

// Reads XML content from a string and converts it to structs according to the schema
fn string_content_to_structs(xml: &str) -> UrlForestPropertyData {
    from_str(&xml).expect("Could not parse the XML")
}

// Converts structs to json and saves them to a json file
fn property_to_json(file_property: Option<FileForestPropertyData>, url_property: Option<UrlForestPropertyData>) {
    if file_property.is_some() {
        let file_property = file_property.unwrap();
/*         let type_name = type_name_of(&file_property);

        let property = serde_json::json!({
            type_name.split("::").last().unwrap(): file_property
        }); */

        match serde_json::to_string_pretty(&file_property) {
            Ok(json) => std::fs::write("file_forestpropertydata.json", &json).expect("Unable to write data"),
            Err(e) => println!("Error: {}", e),
        }
    } 
    
    if url_property.is_some() {
        let url_property = url_property.unwrap();
/*         let type_name = type_name_of(&url_property);

        let property = serde_json::json!({
            type_name.split("::").last().unwrap(): url_property
        }); */

        match serde_json::to_string_pretty(&url_property) {
            Ok(json) => std::fs::write("url_forestpropertydata.json", &json).expect("Unable to write data"),
            Err(e) => println!("Error: {}", e),
        }
    }
}

// Get the type name of a struct
fn type_name_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

// Convert Json to XML
fn json_to_xml(path: &str) {
    let json = fs::read_to_string(path).expect("Could not read the JSON file");
    let json_value: serde_json::Value = serde_json::from_str(&json).unwrap();

    // Create the writer
    let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2); // 2-space indentation

    // Write XML header
    let root = "ForestPropertyData";
    writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None))).expect("Unable to write XML declaration");
    create_xml_element(&json_value, &mut writer, root);

    // Write the closing tag
    writer
        .write_event(Event::End(BytesEnd::new(root)))
        .expect("Unable to write end tag"); 

    let xml_output = String::from_utf8(writer.into_inner().into_inner()).expect("Failed to convert to UTF-8");
    std::fs::write("file_back_to_xml.xml", &xml_output).expect("Unable to write data");
}

fn create_xml_element(json_data: &Value, writer: &mut Writer<Cursor<Vec<u8>>>, parent_tag: &str) {
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
                if value.is_object() {
                    let first_key = value.as_object().unwrap().keys().next().unwrap();
                    let first_value = value.as_object().unwrap().get(first_key).unwrap();

                    // Write the start tag for all elements except the first one
                    if first_value.is_object() && !first_element {
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

// Check if the value is an object, in which case check the first key
