use schema_generator::generate_string::generate_structs_string;
use schema_generator::create_structs::create_structs;
use schema_generator::file_structs::ForestPropertyData as FileForestPropertyData;
use schema_generator::url_structs::ForestPropertyData as UrlForestPropertyData;
use schema_generator::generate_xml::create_xml_element;

use quick_xml::Writer;
use quick_xml::events::{BytesDecl, BytesEnd, Event};
use quick_xml::reader::Reader;
use quick_xml::de::from_str;
use reqwest::blocking::get;
use std::fs::File;
use std::collections::HashMap;
use std::io::{Read, Write, Cursor};
use std::str;
use std::fs;
use regex::Regex;

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

    json_to_xml("file_forestpropertydata.json", "file_back_to_xml.xml");
    json_to_xml("url_forestpropertydata.json", "url_back_to_xml.xml");
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

        match serde_json::to_string_pretty(&file_property) {
            Ok(json) => std::fs::write("file_forestpropertydata.json", &json).expect("Unable to write data"),
            Err(e) => println!("Error: {}", e),
        }
    } 
    
    if url_property.is_some() {
        let url_property = url_property.unwrap();

        match serde_json::to_string_pretty(&url_property) {
            Ok(json) => std::fs::write("url_forestpropertydata.json", &json).expect("Unable to write data"),
            Err(e) => println!("Error: {}", e),
        }
    }
}

// Convert Json to XML
fn json_to_xml(path: &str, file_name: &str) {
    let json = fs::read_to_string(path).expect("Could not read the JSON file");
    let json_value: serde_json::Value = serde_json::from_str(&json).unwrap();

    // Create the writer
    let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2); // 2-space indentation

    // Extract the prefixes from the root element
    let prefixes = extract_prefixes(&json_value);

    for pr in prefixes.iter() {
        println!("{}: {}", pr.0, pr.1);
    }
    println!();
    // Write XML header
    let root = "ForestPropertyData";
    writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None))).expect("Unable to write XML declaration");
    create_xml_element(&json_value, &mut writer, root, &prefixes, &mut "".to_string());

    // Write the closing tag
    writer
        .write_event(Event::End(BytesEnd::new(root)))
        .expect("Unable to write end tag"); 

    let xml_output = String::from_utf8(writer.into_inner().into_inner()).expect("Failed to convert to UTF-8");
    std::fs::write(file_name, &xml_output).expect("Unable to write data");
}

fn extract_prefixes(json_data: &serde_json::Value) -> HashMap<String, String> {
    let mut prefixes: HashMap<String, String> = HashMap::new();

    match json_data {
        serde_json::Value::Object(map) => {
            for (key, value) in map {
                if key.starts_with("@xmlns:") {
                    let prefix = key.split(':').last().unwrap().to_string();
                    let struct_string = value.as_str().unwrap().to_string();

                    // Extract the namespace from the struct string
                    let re = Regex::new(r"/\d{4}/\d{2}/\d{2}").unwrap();
                    let namespace = re.replace(&struct_string, "").to_string();

                    // Extract the last segment of the namespace
                    let last_segment = namespace.split('/').last().unwrap().to_string();
                    let formatted_namespace = capitalize_word(&last_segment);

                    prefixes.insert(formatted_namespace, prefix);
                }
            }
        },
        _ => {}
    }

    prefixes
}

// Capitalizes the first letter of a word
fn capitalize_word(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
