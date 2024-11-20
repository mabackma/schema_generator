use schema_generator::generate_string::generate_structs_string;
use schema_generator::create_structs::create_structs;

use quick_xml::reader::Reader;
use std::fs::File;
use std::io::{Read, Write};
use std::str;

fn main() {    
    let xml_string = read_xml_file("forestpropertydata.xml");
    let mut reader = Reader::from_str(&xml_string);

    let mut structs = create_structs(&mut reader);

    let struct_string = generate_structs_string(&mut structs);

    // Save the generated structs to a file
    let mut struct_file = File::create("src/structs.rs").unwrap();
    struct_file.write_all(&struct_string.as_bytes()).unwrap();
}

// Reads an XML file and returns its contents as a string
fn read_xml_file(file_name: &str) -> String {
    let mut file = File::open(file_name).unwrap();
    let mut xml_string = String::new();
    file.read_to_string(&mut xml_string).unwrap();
    xml_string
}
