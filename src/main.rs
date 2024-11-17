use quick_xml::reader::Reader;
use quick_xml::events::Event::{Start, End, Eof};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};
use xml_schema_generator::{into_struct, Options};

#[derive(Debug)]
struct XMLField {
    name: String,
    field_type: String,
}

#[derive(Debug)]
struct XMLStruct {
    name: String,
    fields: Vec<XMLField>,
}

fn main() {    
    let xml_string = read_xml_file("forestpropertydata.xml");
    let mut reader = Reader::from_str(&xml_string);

/*     if let Ok(root) = into_struct(&mut reader) {
        let struct_as_string = root.to_serde_struct(&Options::quick_xml_de());

        // save this result as a .rs file and use it to (de)serialize an XML document with quick_xml::de::from_str(xml)
        let mut struct_file = File::create("src/forest_property_data.rs").unwrap();
        struct_file.write_all(&struct_as_string.as_bytes()).unwrap();
    } */

    let mut stack: Vec<XMLStruct> = Vec::new(); // Stack of structs being constructed
    let mut structs: HashMap<String, XMLStruct> = HashMap::new(); // Finalized structs

    loop {
        match reader.read_event() {
            Ok(Start(ref e)) => {
                let element_name = std::str::from_utf8(e.name().as_ref()).unwrap().to_string();

                // Create a new struct for this element
                let new_struct = XMLStruct {
                    name: element_name.clone(),
                    fields: Vec::new(),
                };

                // If there's a parent struct, add this struct as a field to it
                if let Some(parent_struct) = stack.last_mut() {
                    // Check that the parent doesn't contain a field with the same name
                    if !parent_struct.fields.iter().any(|field| field.name == element_name) {
                        parent_struct.fields.push(XMLField {
                            name: element_name.clone(),
                            field_type: element_name.clone(), // Use the same name as struct type
                        });
                    }
                }

                // Push this struct onto the stack if it's not already there
                if !stack.iter().any(|s| s.name == element_name) {
                    stack.push(new_struct);
                }
            },
            Ok(End(ref e)) => {
                let element_name = std::str::from_utf8(e.name().as_ref()).unwrap().to_string();

                // Pop the current struct from the stack
                if let Some(completed_struct) = stack.pop() {
                    if completed_struct.name != element_name {
                        panic!("XML structure mismatch: expected {}, found {}", completed_struct.name, element_name);
                    }

                    // Ensure that we either update the struct with new fields or insert it if it doesn't exist
                    if let Some(existing_struct) = structs.get_mut(&completed_struct.name.clone()) {
                        // Merge fields: add only new unique fields
                        for field in completed_struct.fields {
                            if !existing_struct.fields.iter().any(|f| f.name == field.name) {
                                existing_struct.fields.push(field);
                            }
                        }
                    } else {
                        // No existing struct, insert the completed struct as it is
                        structs.insert(completed_struct.name.clone(), completed_struct);
                    }
                }
            },
            Ok(Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
    }

    // Remove structs that don't have any fields
    let keys_to_remove: Vec<String> = structs
        .iter()
        .filter(|(_, xml_struct)| xml_struct.fields.is_empty())
        .map(|(name, _)| name.clone())
        .collect();

    for key in keys_to_remove {
        structs.remove(&key);
    }

    let mut struct_string = String::new();

    // Generate Rust structs from the XML structs
    for (name, xml_struct) in &structs {
        let struct_name = name.split(":").last().unwrap(); 
        struct_string += &format!("#[derive(Serialize, Deserialize)]\n");
        struct_string += &format!("pub struct {} {{\n", struct_name);

        for field in &xml_struct.fields {
            let mut field_type = "";

            // Check if the field type is a struct
            if structs.contains_key(&field.field_type) {
                field_type = field.name.split(":").last().unwrap();
            } else {
                field_type = "String";
            }

            let field_name = field.name.split(":").next().unwrap().to_owned() + "_" + to_snake_case(&field_type).as_str();
            struct_string += &format!("\t#[serde(rename = \"{}\", skip_serializing_if = \"Option::is_none\")]\n", field_type);
            struct_string += &format!("\tpub {}: Option<{}>,\n", field_name, field_type);
        }
        struct_string += &format!("}}\n");
        struct_string += &format!("\n");
    }

    // Save the generated structs to a file
    let mut struct_file = File::create("src/structs.rs").unwrap();
    struct_file.write_all(&struct_string.as_bytes()).unwrap();
}

fn to_snake_case(s: &str) -> String {
    let char_vec: Vec<char> = s.chars().collect();
    let mut new_string  = String::new();

    for c in char_vec {
        if c.is_uppercase() && new_string.len() > 0 {
            new_string.push('_');
            new_string.push(c.to_lowercase().next().unwrap());
        } else {
            new_string.push(c);
        }
    }

    new_string.to_lowercase()
}

fn read_xml_file(file_name: &str) -> String {
    let mut file = File::open(file_name).unwrap();
    let mut xml_string = String::new();
    file.read_to_string(&mut xml_string).unwrap();
    xml_string
}
