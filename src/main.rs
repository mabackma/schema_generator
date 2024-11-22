use schema_generator::generate_string::generate_structs_string;
use schema_generator::create_structs::create_structs;
use schema_generator::file_structs::ForestPropertyData as FileForestPropertyData;
use schema_generator::url_structs::ForestPropertyData as UrlForestPropertyData;

use quick_xml::reader::Reader;
use quick_xml::de::from_str;
use reqwest::blocking::get;
use std::fs::File;
use std::io::{Read, Write};
use std::str;
use std::fs;

fn main() {    
    // Create structs from a file
    let file_xml_string = read_xml_file("forestpropertydata.xml");
    create_structs_and_save_to_file(&file_xml_string, "src/file_structs.rs");

    // Convert the file XML content to structs and save them to a file
    let file_property = file_content_to_structs("forestpropertydata.xml");
    match serde_json::to_string_pretty(&file_property) {
        Ok(json) => json_to_file("file_forestpropertydata.json", &json),
        Err(e) => println!("Error: {}", e),
    }

    // Create structs from a URL
    let url = "https://avoin.metsakeskus.fi/rest/mvrest/FRStandData/v1/ByPolygon?wktPolygon=POLYGON%20((393960.156%206801453.126,%20394798.608%206801657.878,%20394930.512%206801670.111,%20395028.723%206802116.858,%20394258.945%206801929.148,%20394261.711%206801810.541,%20394091.166%206801665.961,%20393960.156%206801453.126))&stdVersion=MV1.9";
    let url_xml_string = fetch_xml_url(url).unwrap();
    create_structs_and_save_to_file(&url_xml_string, "src/url_structs.rs");

    // Convert the URL XML content to structs and save them to a file
    let url_property = string_content_to_structs(&url_xml_string);
    match serde_json::to_string_pretty(&url_property) {
        Ok(json) => json_to_file("url_forestpropertydata.json", &json),
        Err(e) => println!("Error: {}", e),
    }
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

// Creates json from structs
fn json_to_file(file_name: &str, data: &str) {
    let mut file = File::create(file_name).expect("Unable to create file");
    file.write_all(data.as_bytes()).expect("Unable to write data");
}
