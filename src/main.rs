use quick_xml::reader::Reader;
use quick_xml::events::Event::{Start, End, Eof};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};
use std::str;

#[derive(Debug, Clone)]
struct XMLField {
    name: String,
    field_type: String,
}

#[derive(Debug)]
struct XMLStruct {
    name: String,
    fields: Vec<XMLField>,
}

impl Clone for XMLStruct {
    fn clone(&self) -> Self {
        XMLStruct {
            name: self.name.clone(),
            fields: self.fields.clone(),
        }
    }
}

fn main() {    
    let xml_string = read_xml_file("forestpropertydata.xml");
    let mut reader = Reader::from_str(&xml_string);

    let mut structs: HashMap<String, XMLStruct> = HashMap::new(); // Finalized structs

    create_structs(&mut reader, &mut structs);

    let struct_string = generate_structs_string(&mut structs);

    // Save the generated structs to a file
    let mut struct_file = File::create("src/structs.rs").unwrap();
    struct_file.write_all(&struct_string.as_bytes()).unwrap();
}

// Create structs from the XML document
fn create_structs(reader: &mut Reader<&[u8]>, structs: &mut HashMap<String, XMLStruct>) {
    let mut stack: Vec<XMLStruct> = Vec::new(); // Stack of structs being constructed
    let mut field_counts: HashMap<String, HashMap<String, usize>> = HashMap::new(); // Count of fields per struct
 
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
                    // Count the number of fields with the same name
                    let parent_name = parent_struct.name.clone();
                    let field_count = field_counts.entry(parent_name).or_insert(HashMap::new());
                    let child_count = field_count.entry(element_name.clone()).or_insert(0);
                    *child_count += 1; // Because child_count and field_count are borrowed, field_counts is updated after this line

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

                    // Update the final struct with new fields or insert it if it doesn't exist
                    if let Some(existing_struct) = structs.get_mut(&completed_struct.name.clone()) {
                        // Merge fields: add only new unique fields
                        for field in completed_struct.fields {
                            if !existing_struct.fields.iter().any(|f| f.name == field.name) {
                                existing_struct.fields.push(field.clone());
                            }

                            // Reset the field_count for this field
                            if field_counts.contains_key(&field.name) {
                                field_counts.get_mut(&field.name).unwrap().insert(field.name, 0);
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
 
    remove_fieldless_structs(structs);

    let struct_keys: Vec<String> = structs.keys().cloned().collect();

    // Update structs to use Vec<T> for fields with multiple occurrences
    for (struct_name, counts) in field_counts {
        if let Some(xml_struct) = structs.get_mut(&struct_name) {
            // Update field types to Vec<T> if the field occurs more than once
            for field in &mut xml_struct.fields {
                if let Some(count) = counts.get(&field.name) {
                    if *count > 1 {
                        if struct_keys.contains(&field.name) {
                            field.field_type = format!("Vec<{}>", field.name);
                        }
                    } 
                    println!("{} ------- {}", field.name, field.field_type);
                }
            }
        }
    }
}

// Removes structs that don't have any fields
fn remove_fieldless_structs(structs: &mut HashMap<String, XMLStruct>) {
    let keys_to_remove: Vec<String> = structs
        .iter()
        .filter(|(_, xml_struct)| xml_struct.fields.is_empty())
        .map(|(name, _)| name.clone())
        .collect();

    for key in keys_to_remove {
        structs.remove(&key);
    }

}

// Generates String of Rust structs with fields 
fn generate_structs_string(structs: &HashMap<String, XMLStruct>) -> String {
    let mut struct_string = String::new();

    for (name, xml_struct) in structs {
        let struct_name = prefix_to_camel_case(&name); 
        struct_string += &format!("#[derive(Serialize, Deserialize)]\n");
        struct_string += &format!("pub struct {} {{\n", struct_name);

        for field in &xml_struct.fields {
            let mut field_type = "";
            let mut field_type_string = String::new();

            println!("-------------------------------------{}", field.field_type);
            // Check if the field type is a struct
            if (*structs).contains_key(remove_vec(&field.field_type).as_str()) {
                field_type_string = prefix_to_camel_case(&field.field_type);
                println!("{}: {}", field.field_type, field_type_string);
                field_type = field_type_string.as_str();
            } else {
                field_type = "String";
            }

            let renaming = field.name.split(":").last().unwrap();
            let field_name = field.name.split(":").next().unwrap().to_owned() + "_" + to_snake_case(&renaming).as_str();
            struct_string += &format!("\t#[serde(rename = \"{}\", skip_serializing_if = \"Option::is_none\")]\n", renaming);
            struct_string += &format!("\tpub {}: Option<{}>,\n", field_name, field_type);
        }
        struct_string += &format!("}}\n");
        struct_string += &format!("\n");
    }

    struct_string
}

fn remove_vec(s: &str) -> String {
    let mut new_string = String::new();

    if s.starts_with("Vec<") {
        // Remove Vec< and >
        new_string = s.chars().skip(4).take(s.len() - 5).collect();
    } else {
        new_string = s.to_string();
    }

    new_string
}

fn prefix_to_camel_case(s: &str) -> String {
    let mut new_string = String::new();
    let mut char_vec: Vec<char>;

    if s.starts_with("Vec<") {
        new_string.push_str("Vec<");
        for c in s.chars().skip(4).take(s.len() - 5) {
            if c == ':' {
                continue;
            } else {
                new_string.push(c);
            }
        }
        new_string.push('>');

        char_vec = new_string.chars().collect();
        char_vec[4] = char_vec[4].to_uppercase().next().unwrap();
    } else {
        for c in s.chars() {
            if c == ':' {
                continue;
            } else {
                new_string.push(c);
            }
        }

        char_vec = new_string.chars().collect();
        char_vec[0] = char_vec[0].to_uppercase().next().unwrap();
    }

    char_vec.into_iter().collect()
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
